use super::Mode;
use crate::{ Image, Serializable};

use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct PGM {
    mode: Mode,
    width: u32,
    height: u32,
    max_value: u8,
    data: Vec<u8>,
}

impl PGM {
    pub fn new(mode: Mode, width: u32, height: u32, max_value: u8, data: &Vec<u8>) -> Self {
        Self {
            mode,
            width,
            height,
            max_value,
            data: data.to_vec(),
        }
    }
}

impl Serializable for PGM {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self.mode {
            Mode::Ascii => {
                let header = format!(
                    "{magic_number}\n{width} {height}\n{max_value}\n",
                    magic_number = "P2",
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
            }
            Mode::Binary => {
                let header = format!(
                    "{magic_number}\n{width} {height}\n{max_value}\n",
                    magic_number = "P5",
                    width = self.width,
                    height = self.height,
                    max_value = self.max_value
                );
                bytes.extend(header.bytes());
                // Map the gray value from [0, max_value] to [0, 255]
                let grays = self
                    .data
                    .iter()
                    .map(|x| x * (256 / (self.max_value as u16 + 1) as u16) as u8)
                    .collect::<Vec<_>>();
                bytes.extend(grays);
            }
        }

        bytes
    }
}

impl Image for PGM {
    fn dump(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        let bytes = self.get_bytes();
        file.write(&bytes)?;
        Ok(())
    }
}
