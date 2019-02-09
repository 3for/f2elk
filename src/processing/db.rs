use std::collections::HashMap;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::fs::File;
use std::cell::Cell;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileParams {
    pub offset: Cell<u64>,
    pub modified: Cell<u64>,
}

#[derive(Debug)]
pub  struct  DataBase <'a> {
    file_name: &'a str,
    files: HashMap<String, FileParams>,
}
impl <'a> Drop for DataBase <'a> {
    fn drop(& mut self) {
        self.sync_to_disk();
    }
}
impl <'a>DataBase<'a> {
    pub fn new(file_name: &str) -> DataBase{
        let files:HashMap<String, FileParams> = HashMap::new();
        DataBase {
            file_name: &file_name,
            files,
        }
    }

    pub fn read(&mut self){
        let f = match File::open("db.json") {
            Ok(r) => r,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    self.sync_to_disk();
                    return ;
                },
                error => panic!("{:?}", error),
            },
        };
        let reader = BufReader::new(f);

        let des: HashMap<String, FileParams> = serde_json::
            from_reader(reader)
            .unwrap();
        self.files.extend(des);

    }
    pub fn get(&mut self, file_name: &str) -> (&Cell<u64>, &Cell<u64>) {
        let r = self.files.
            entry(file_name.to_string()).
            or_insert(FileParams{offset: Cell::new(0), modified: Cell::new(0)});
        (&r.offset, &r.modified)

    }
    #[allow(dead_code)]
    pub fn update(&mut self,
                  file_name: &str, offset: u64, modified: u64) {
        let f = self.files.entry(file_name.to_string())
            .or_insert(FileParams{offset: Cell::new(0), modified: Cell::new(0)});
        f.offset.set(offset);
        f.modified.set(modified);
    }
    pub fn sync_to_disk(&mut self){
        let f = File::create("db.json").unwrap();
        let writer = BufWriter::new(f);
        serde_json::to_writer(writer, &self.files).expect("foo");
    }
}

