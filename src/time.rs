use std::time::SystemTime;

pub trait Seconds {
        fn seconds(&self) -> u64;
}

impl Seconds for SystemTime{
    fn seconds(&self) -> u64 {
        self.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }
}

