//#![allow(dead_code, unused_impors)]
#[macro_use] extern crate serde_derive;

extern crate regex;
extern crate serde;
extern crate serde_json;
use std::rc::Rc;

mod config;
use config::config_reader;
mod time;
mod file;
mod export;
mod processing;
use processing::process_single_file;


fn main_loop(){
    let config = Rc::new(config_reader("config.json"));
    let files = file::file::get_file_by_pattern(
        &config.path, &config.file_pattern).unwrap();
    let exporter = export::stdout::StdoutSender{counter:0};
    for file in &files{
        process_single_file(&file, &config.db_file, exporter);
    }
}
fn main() -> Result<(), std::io::Error> {
    main_loop();
    Ok(())
}

