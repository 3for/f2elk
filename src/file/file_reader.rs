use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::BufReader;
use std::fs::File;


pub struct RecordIter {
    reader: BufReader<File>,
}
pub struct LogRecord {
    pub offset: u64,
    pub data: String,
}

impl Iterator for RecordIter  {
    type Item = LogRecord;

    fn next(&mut self) -> Option<LogRecord>{

        let mut buf = vec![];
        let len = self.reader
            .read_until(b'\n', &mut buf).unwrap();
        let offset = self.reader
            .seek(SeekFrom::Current(0)).unwrap();

        if len == 0 {
            return None;
        }
        if buf.contains(&b'\0') {
            return None;
        }
        let line = String::from_utf8_lossy(&buf);
        let line = line.trim();

        let rec = LogRecord{
            offset,
            data: line.into(),
        };
        Some(rec)

    }

}

pub struct  Lines<'a> {
    file_name: &'a str,
}

impl <'a> Lines <'a>{
    pub fn new(file_name: &str) -> Lines{
        Lines {file_name}
    }
    pub fn iter(&self, offset: u64) -> RecordIter{
        let f = File::open(self.file_name).expect("File not found");
        let mut reader = BufReader::new(f);
        let _offset = reader.seek(SeekFrom::Start(offset)).unwrap();
        RecordIter {
            reader,
        }
    }
}

//impl <'a> IntoIterator for &'a Lines<'a> {
//    type Item = LogRecord;
//    type IntoIter = RecordIter;
//    fn into_iter(self) -> RecordIter {
//        self.iter()
//    }
//}
