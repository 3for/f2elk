use std::io::{self, Write};
use super::*;

#[derive(Clone, Copy)]
pub struct StdoutSender{
    pub counter: i64,
}

impl Exporter for StdoutSender {
    fn send(&self, file_name: &str, line: &str, offset :u64) -> Result<(), String>{
        let f = move || -> Result<(), io::Error>{
            let stdout = io::stdout();
            let mut w = stdout.lock();
            w.write_fmt(format_args!("{} - {} - {}\n", file_name, line, offset))?;
            // TODO: w.flush()?;
            Ok(())
        };
        if let Err(e) = f() {
            match e.kind() {
                io::ErrorKind::BrokenPipe => {return Err("BrokenPipe".to_string())},
                _ => panic!(e),
            }
        }
        Ok(())
    }
}

