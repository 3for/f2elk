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
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

mod config;
use config::config_reader;
mod file_reader;
mod time;
mod file;
mod db;
use file_reader::Lines;


fn reader(file_name: &str, offset: u64, tx: SyncSender<(String,u64)>) -> (&str, u64) {
    let a = "tst";
    let b = 42;
    (a,b)

}


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

fn main() -> Result<(), std::io::Error> {

    let running = Arc::new(Mutex::new(true));
    let running_in_handler = running.clone();

    let config = Rc::new(config_reader("config.json"));
    let config1 = config.clone();
    let offsets_db = Arc::new(RefCell::new(db::DataBase::new(&config1.db_file)));

    ctrlc::set_handler(move || {
        let mut r = running_in_handler.lock().unwrap();
        *r = false;

    }).expect("Error setting Ctrl-C handler");


    match run(config, offsets_db, running.clone()) {
        Ok(()) => {},
        Err(_e) => {exit(0);}
    }


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
