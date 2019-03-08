use std::thread;
use std::sync::mpsc::{SyncSender};
use file::file_reader::Lines;

pub fn lines_producer(
    offset: &std::cell::Cell<u64>,
    file_name: &str,
    line_sender: SyncSender<(String, u64)>)
-> std::thread::JoinHandle<(bool)>
{
    let file_name = file_name.to_owned();
    let sender_handler = thread::Builder::new()
        .name("sender".to_string()).spawn({
    let offset = offset.get().clone();

    move || {
        let mut finished = true;
        let lines = Lines::new(&file_name);
        if let Ok(iter) = lines.iter(offset) {
            for line in iter{
                 if let Err(_) = line_sender
                     .send((line.data, line.offset)) {
                         finished=false;
                         break;
                     };
            }

        }
        else {
            finished = false;
            eprintln!("Cant read file:{}. Skipping", &file_name);
        }
        finished
    }}).unwrap();
    return sender_handler
}

