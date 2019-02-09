use regex::Regex;
use std::fs::read_dir;
use std::{self};
use time::Seconds;

#[derive(Debug)]
pub struct File{
    pub file_name: String,
    pub modified: u64,
}

pub fn get_file_by_pattern (
        path: &str, pattern: &str) -> Result<Vec<File>, std::io::Error>{
    let mut files:Vec<File> = Vec::new();
    let dir = read_dir(&path)?;
    let re = Regex::new(&pattern).unwrap();

    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata().unwrap();

        let is_match = re.is_match(path.file_name().unwrap().to_str().unwrap());

        if !path.is_dir() && is_match {
            let f = File{
                file_name: path.canonicalize()?
                    .into_os_string().into_string().unwrap(),
                modified: metadata.modified()?.seconds(),
            };
            files.push(f);
        }
    }
    Ok(files)
}
