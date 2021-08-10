mod checksum;
pub mod netpbm;
pub mod png;
pub mod jpg;

use std::error::Error;
use std::fs::File;
use std::io::Write;

pub trait Serializable {
    fn get_bytes(&self) -> Vec<u8>;
}

pub trait Image: Serializable {
    fn dump(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        let bytes = self.get_bytes();
        file.write(&bytes)?;
        Ok(())
    }
}
