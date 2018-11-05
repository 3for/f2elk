pub mod stdout;

pub trait Exporter{
    fn send(&self, file_name: &str, line: &str, offset: u64) -> Result<(), &'static str> ;
}

