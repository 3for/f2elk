//#![allow(dead_code, unused_impors)]
#[macro_use] extern crate serde_derive;

extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate ctrlc;
use std::rc::Rc;
use std::sync::{Arc};
use std::sync::mpsc::{sync_channel, SyncSender,Receiver};
use std::thread;

mod config;
use config::config_reader;
mod file_reader;
mod time;
mod file;
mod db;
mod export;
use file_reader::Lines;


fn reader(
    file_name: &str,
    offset: u64,
    tx: SyncSender<(String, u64)>) {

    let lines = Lines::new(&file_name);
    for line in lines.iter(offset){
         if let Err(_) = tx.send((line.data, line.offset)) {break;};
    }
}
fn sender<T: export::Exporter>(
    s: T, file_name: &str,
    rx: Receiver<(String, u64)>,
    offset_sender: SyncSender<u64>){
    while let Ok((line, offset)) = rx.recv() {
        match s.send(file_name, &line, offset){
            Ok(_) => {
                if let Err(_) = offset_sender.send(offset) {break;}
            },
            Err("BrokenPipe") => break,
            Err(e) => panic!(e),
        }
    }
}

fn run2() {
    let config = Rc::new(config_reader("config.json"));
    let files = file::get_file_by_pattern(
        &config.path, &config.file_pattern).unwrap();
    let file_sender = export::stdout::StdoutSender{};

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
        let file_name2= file_name.clone();
        let offset_for_sender = offset.get().clone();

        let sender_handler = thread::Builder::new().name("sender".to_string()).spawn(move || {
            reader(&file_name.clone(), offset_for_sender , line_sender);
        }).unwrap();
        let receiver_handler = thread::Builder::new().name("receiver".to_string()).spawn(move || {
            sender(&file_sender, &file_name2.clone(), line_receiver, offset_sender);
        }).unwrap();
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

