use crate::lib::Image;

use std::fs::File;
use std::io::Write;
use std::error::Error;

pub struct PGM {
    magic_number: &'static str,
    width: u32,
    height: u32,
    max_value: u32,
    data: Vec<u8>,
}

impl PGM {
    pub fn new(width: u32, height: u32, max_value: u32, data: &Vec<u8>) -> Self {
        Self {
            magic_number: "P2",
            width,
            height,
            max_value,
            data: data.to_vec(),
        }
    }
}

impl Image for PGM {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        let header = format!(
            "{magic_number}\n{width} {height}\n{max_value}\n",
            magic_number = self.magic_number,
            width = self.width,
            height = self.height,
            max_value = self.max_value
        );

        bytes.extend(header.bytes());

        let mut data = String::new();
        for gray_value in &self.data {
            data += &gray_value.to_string();
            data += " ";
        }
        bytes.extend(data.bytes());

        bytes
    }

    fn dump(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        let bytes = self.get_bytes();
        file.write(&bytes)?;
        Ok(())
    }
}