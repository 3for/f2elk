use std::thread;
use std::sync::mpsc::{SyncSender, Receiver};
use export::Exporter;
pub fn lines_sender<T>(file_name: &str, line_receiver: Receiver<(String, u64)>, offset_sender: SyncSender<u64>, exporter: T)
    -> std::thread::JoinHandle<()>
    where T: Exporter + Send + 'static

{
   let file_name = file_name.to_owned();
   let receiver_handler  = thread::Builder::new()
            .name("receiver".to_string()).spawn({
        let file_name = file_name.clone();
        move || {
        while let Ok((line, offset)) = line_receiver.recv() {
            match exporter.send(&file_name, &line, offset){
                Ok(_) => {
                    if let Err(_) = offset_sender.send(offset) {break;}
                },
                //Err("BrokenPipe") => break,
                Err(e) => panic!(e),
            }
        }

    }}).unwrap();
   receiver_handler
}

