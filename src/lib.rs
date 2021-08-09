mod netpbm;
mod checksum;
mod png;
mod jpg;

use std::error::Error;

pub trait Serializable {
    fn get_bytes(&self) -> Vec<u8>;
}

pub trait Image {
    fn dump(&self, path: &str) -> Result<(), Box<dyn Error>>;
}
