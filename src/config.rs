use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct SslConfig {
    pub client_pem_file: String,
    pub client_ca_chain_file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub file_pattern: String,
    pub path: String,
    pub db_file: String,
    pub logstash_connection: String,
    pub logstash_ssl: Option<SslConfig>,
}
impl Config {
    pub fn from_file(config_path: &str) -> Result<Self, Error> {
        let mut f = File::open(config_path)?;
        let mut config_raw = String::new();

        f.read_to_string(&mut config_raw)?;
        let config: Config = serde_json::from_str(&config_raw)?;
        Ok(config)
    }
}

