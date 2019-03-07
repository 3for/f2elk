//pub mod stdout;
pub mod https;

pub trait Exporter {
    fn send(&self, file_name: &str, lines: Vec<(String, u64)>) -> Result<(), String> ;
}

