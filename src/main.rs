//#![allow(dead_code, unused_impors)]
#[macro_use] extern crate serde_derive;

extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate ctrlc;
use std::rc::Rc;

mod config;
use config::config_reader;
mod time;
mod file;
mod export;
mod processing;
use processing::process_single_file;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn sig_handler() -> Arc<AtomicBool> {
    let term = Arc::new(AtomicBool::new(false));
    let r = term.clone();
    ctrlc::set_handler(move || {
        r.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    term
}

fn main_loop(){
    let term = sig_handler();
    let config = Rc::new(config_reader("config.json"));
    let files = file::file::get_file_by_pattern(
        &config.path, &config.file_pattern).unwrap();
    //let exporter = export::stdout::StdoutSender{counter:0};
    let exporter = export::https::HttpsSender::new(
        "https://logstash.fortfs.net:5048",
        "certs/client_bundle.pem",
        "certs/client.chain"
    ).unwrap();
    for file in &files{
        process_single_file(&file, &config.db_file, exporter.to_owned(), &term);
    }
}
fn main() -> Result<(), std::io::Error> {
    main_loop();
    Ok(())
}

