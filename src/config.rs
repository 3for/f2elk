use std::io::prelude::*;
use std::fs::File;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub file_pattern: String,
    pub path: String,
    pub db_file: String,
}


pub fn config_reader(config_path: &'static str) -> Config {
    let mut f = File::open(config_path)
        .expect("can't open config file");
    let mut config_raw = String::new();

    f.read_to_string(&mut config_raw)
        .expect("can't read config file ");
    let config: Config = serde_json::from_str(&config_raw)
        .unwrap();
    config
}
