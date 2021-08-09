use crate::lib::Image;
use super::Mode;

use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct PGM {
    magic_number: &'static str,
    width: u32,
    height: u32,
    max_value: u8,
    data: Vec<u8>,
    mode: Mode
}

impl PGM {
    pub fn new(width: u32, height: u32, max_value: u8, mode: Mode, data: &Vec<u8>) -> Self {
        Self {
            magic_number: if let Mode::Ascii = mode { "P2" } else { "P5" },
            width,
            height,
            max_value,
            data: data.to_vec(),
            mode
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

        if let Mode::Ascii = self.mode {
            let mut data = String::new();
            for gray_value in &self.data {
                data += &gray_value.to_string();
                data += " ";
            }
            bytes.extend(data.bytes());
        } else {
            let grays = self.data.iter().map(|x| x * (256 / (self.max_value + 1) as u16) as u8).collect::<Vec<_>>();
            bytes.extend(grays);
        }

        bytes
    }

    fn dump(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        let bytes = self.get_bytes();
        file.write(&bytes)?;
        Ok(())
    }
}
