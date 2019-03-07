mod db;
mod lines_sender;
mod lines_producer;

use std::sync::mpsc::{sync_channel};
use super::export::Exporter;
use super::file::file::File;
use self::lines_producer::lines_producer;
use self::lines_sender::lines_sender;
use std::sync::atomic::AtomicBool;


use std::io::Error;
use std::sync::Arc;


pub fn process_single_file<T>(file:&File, db_file: &str, exporter: T, term: &Arc<AtomicBool>)
where T: Exporter + Send + 'static

{
    let mut offsets_db = db::DataBase::new(db_file);
    offsets_db.read();
    let (offset, modified) = offsets_db.get(&file.file_name);

    if modified.get() >= file.modified {
        return;
    }

    let (line_sender, line_receiver) = sync_channel(1000);
    let (offset_sender, offset_receiver) = sync_channel(1000);
    let file_name = file.file_name.clone();

    let sender_handler = lines_producer(offset, &file_name, line_sender);
    let receiver_handler = lines_sender(&file_name, line_receiver, offset_sender, exporter, term);
    while let Ok(r) = offset_receiver.recv(){
            offset.set(r);
    }

    receiver_handler.join().unwrap();
    sender_handler.join().unwrap();
}

