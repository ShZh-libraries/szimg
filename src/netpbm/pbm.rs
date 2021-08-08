use crate::lib::Image;

use std::fs::File;
use std::io::Write;
use std::error::Error;

pub struct PBM {
    magic_number: &'static str,
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl PBM {
    pub fn new(width: u32, height: u32, data: &Vec<u8>) -> Self {
        Self {
            magic_number: "P1",
            width,
            height,
            data: data.to_vec(),
        }
    }
}

impl Image for PBM {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let header = format!(
            "{magic_number}\n{width} {height}\n",
            magic_number = self.magic_number,
            width = self.width,
            height = self.height,
        );
        bytes.extend(header.bytes());
        bytes.extend(&self.data);

        bytes
    }

    fn dump(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        let bytes = self.get_bytes();
        file.write(&bytes)?;
        Ok(())
    }
}