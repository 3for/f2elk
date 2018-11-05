#![allow(dead_code)]
#[macro_use] extern crate serde_derive;

extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate ctrlc;
use std::io::{self, Write};
use std::process::exit;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Mutex,Arc};
use std::sync::mpsc::{sync_channel, channel, SyncSender,Sender, Receiver};
use std::thread;

mod config;
use config::config_reader;
mod file_reader;
mod time;
mod file;
mod db;
use file_reader::Lines;


fn reader(file_name: &str, offset: u64, tx: SyncSender<(String, u64)>) {
    let lines = Lines::new(&file_name);
    for line in lines.iter(offset){
        tx.send((line.data, line.offset));
    }
}
fn sender(file_name: &str, rx: Receiver<(String, u64)>, offset_sender: SyncSender<u64>){
    loop {
        let (line, offset) = match rx.recv() {
            Ok(r) => r,
            Err(_) => break,
        };
        let stdout = io::stdout();
        let mut w = stdout.lock();
        match w.write_fmt(format_args!("{} - {}\n", line, offset)) {
            Ok(_) => {
                offset_sender.send(offset);
            },
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            },
        }
    }
}

fn run2(){
    let config = Rc::new(config_reader("config.json"));
    let files = file::get_file_by_pattern(&config.path, &config.file_pattern).unwrap();
    for file in &files{
        let mut offsets_db = db::DataBase::new(&config.db_file);
        offsets_db.read();
        let (offset, modified) = offsets_db.get(&file.file_name);
        eprintln!("{:?}", offset);

        if modified.get() >= file.modified {
            break;
        }

        let (line_sender, line_receiver) = sync_channel(1000);
        let (offset_sender, offset_receiver) = sync_channel(1000);
        let file_name = Arc::new(file.file_name.clone());
        let file_name2= file_name.clone();
        let offset_for_sender = offset.get().clone();

        let sender_handler = thread::spawn(move || {
            reader(&file_name, offset_for_sender , line_sender);
        });
        let receiver_handler = thread::Builder::new().name("receiver".to_string()).spawn(move || {
            sender(&file_name2, line_receiver, offset_sender);
        }).unwrap();
        loop{
            match offset_receiver.recv(){
                Ok(r) => {offset.set(r);},
                Err(e) => {break},
            }
        }

        let r = match receiver_handler.join(){
            Ok(r) => {},
            Err(e) => eprintln!("{:?}", e),
        };

        sender_handler.join();
    }
}

fn main() -> Result<(), std::io::Error> {
    //sender_handler.join();
    //let running = Arc::new(Mutex::new(true));
    //let running_in_handler = running.clone();

    //let config = Rc::new(config_reader("config.json"));
    //let config1 = config.clone();
    //let offsets_db = Arc::new(RefCell::new(db::DataBase::new(&config1.db_file)));

    //ctrlc::set_handler(move || {
    //    let mut r = running_in_handler.lock().unwrap();
    //    *r = false;

    //}).expect("Error setting Ctrl-C handler");


    //match run(config, offsets_db, running.clone()) {
    //    Ok(()) => {},
    //    Err(_e) => {exit(0);}
    //}
    let run_handler = thread::spawn(move || {
        run2();
    });
    run_handler.join();

    Ok(())

}
#[derive(Serialize, )]
struct LogstashFields{
   program: String,
}
#[derive(Serialize, )]
struct LogstashLogRecord {
    message: String,
    source: String,
    fields: LogstashFields,
    offset: u64,
}
/*
fn run(config: Rc<config::Config>, offsets_db: Arc<RefCell<db::DataBase>>,
       running: Arc<Mutex<bool>>
       ) -> Result<(), std::io::Error> {
    let mut offsets_db = offsets_db.borrow_mut();
    offsets_db.read();
    let files = file::get_file_by_pattern(&config.path, &config.file_pattern).unwrap();

    'l: for file in &files {

        let (offset, modified) = offsets_db.get(&file.file_name);
        if  file.modified <= modified.get() {
            continue;
        }
        let lines = Lines::new(&file.file_name);
        for rec in lines.iter(offset.get()){
            let r = running.lock().unwrap();
            if *r == false{
                break 'l;
            }
            offset.set(rec.offset);
            let mut result = rec.data.into_bytes();
            result.push(13);
            result.push(10);
            let _r = io::stdout().write(&result)?;

        }
        modified.set(file.modified);
    }
    Ok(())


}
*/
