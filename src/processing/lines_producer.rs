use std::thread;
use std::sync::mpsc::{SyncSender};
use file::file_reader::Lines;

pub fn lines_producer(offset: &std::cell::Cell<u64>, file_name: &str, line_sender: SyncSender<(String, u64)>)
    -> std::thread::JoinHandle<()>
{
    let file_name = file_name.to_owned();
    let sender_handler = thread::Builder::new()
        .name("sender".to_string()).spawn({
    let offset = offset.get().clone();

    move || {
        let lines = Lines::new(&file_name);
        for line in lines.iter(offset){
             if let Err(_) = line_sender
                 .send((line.data, line.offset)) {break;};
        }
    }}).unwrap();
    return sender_handler
}

