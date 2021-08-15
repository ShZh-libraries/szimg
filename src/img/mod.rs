pub mod netpbm;
pub mod png;
pub mod jpg;
pub mod gif;
pub mod bmp;
pub mod tiff;
pub mod avif;

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
