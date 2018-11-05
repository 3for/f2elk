//#![allow(dead_code, unused_impors)]
#[macro_use] extern crate serde_derive;

extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate ctrlc;
use std::rc::Rc;
use std::sync::{Arc};
use std::sync::mpsc::sync_channel;
use std::thread;

mod config;
use config::config_reader;
mod file_reader;
mod time;
mod file;
mod db;
mod export;
use file_reader::Lines;
use export::Exporter;


fn run2() {
    let config = Rc::new(config_reader("config.json"));
    let files = file::get_file_by_pattern(
        &config.path, &config.file_pattern).unwrap();
    let exporter = export::stdout::StdoutSender{};

    for file in &files{
        let mut offsets_db = db::DataBase::new(&config.db_file);
        offsets_db.read();
        let (offset, modified) = offsets_db.get(&file.file_name);

        if modified.get() >= file.modified {
            continue;
        }

        let (line_sender, line_receiver) = sync_channel(10);
        let (offset_sender, offset_receiver) = sync_channel(10);
        let file_name = Arc::new(file.file_name.clone());

        let sender_handler =
                thread::Builder::new()
                    .name("sender".to_string()).spawn({
            let offset = offset.get().clone();
            let file_name = file_name.clone();

            move || {
            let lines = Lines::new(&file_name);
            for line in lines.iter(offset){
                 if let Err(_) = line_sender
                     .send((line.data, line.offset)) {break;};
            }
        }}).unwrap();

        let receiver_handler =
                thread::Builder::new()
                    .name("receiver".to_string()).spawn({
            let file_name = file_name.clone();
            move || {
            while let Ok((line, offset)) = line_receiver.recv() {
                match exporter.send(&file_name, &line, offset){
                    Ok(_) => {
                        if let Err(_) = offset_sender.send(offset) {break;}
                    },
                    Err("BrokenPipe") => break,
                    Err(e) => panic!(e),
                }
            }

        }}).unwrap();
        while let Ok(r) = offset_receiver.recv(){
                offset.set(r);
        }

        receiver_handler.join().unwrap();
        sender_handler.join().unwrap();
    }
}

fn main() -> Result<(), std::io::Error> {
    run2();
    Ok(())
}

