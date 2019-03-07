use std::thread;
use std::sync::mpsc::{SyncSender, Receiver};
use export::Exporter;


use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn take_n<I, K>(iter: I, n: usize) -> Option<Vec<K>>
    where
        I: Iterator<Item = K>
{
    let mut v = Vec::new();
    for  i in iter{
        v.push(i);
        if v.len() == n {
            return Some(v)
        }
    }
    return if v.len()>0 {Some(v)} else {None};
}

pub fn lines_sender<T>(
    file_name: &str, line_receiver: Receiver<(String, u64)>,
    offset_sender: SyncSender<u64>, exporter: T, term: Arc<AtomicUsize>)
    -> std::thread::JoinHandle<()>
    where T: Exporter + Send + 'static

{
   let file_name = file_name.to_owned();
   let receiver_handler  = thread::Builder::new()
            .name("receiver".to_string()).spawn({
        let file_name = file_name.clone();
        move || {
			while let Some(lines) = take_n(line_receiver.iter(), 200){

                if term.load(Ordering::Relaxed) != 0 {println!("breaking by signal"); break;}

                let (_, offset) = lines.last().unwrap().to_owned();
                match exporter.send(&file_name, lines.to_owned()){
                    Ok(_) => {
                        if let Err(_) = offset_sender.send(offset) {break;}
                    },
                    Err(e) => panic!(e),
                }

			}
        }
    }).unwrap();
   receiver_handler
}

