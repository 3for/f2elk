use std::io::{self, Write};
use super::*;

#[derive(Clone, Copy)]
pub struct StdoutSender {}

impl<'a> Exporter for &'a StdoutSender {
    fn send(&self, file_name: &str, line: &str, offset :u64) -> Result<(), &'static str>{
        let f = move || -> Result<(), io::Error>{
            let stdout = io::stdout();
            let mut w = stdout.lock();
            w.write_fmt(format_args!("{} - {} - {}\n", file_name, line, offset))?;
            // TODO: w.flush()?;
            Ok(())
        };
        if let Err(e) = f() {
            match e.kind() {
                io::ErrorKind::BrokenPipe => {return Err("BrokenPipe")},
                _ => panic!(e),
            }
        }
        Ok(())
    }
}

