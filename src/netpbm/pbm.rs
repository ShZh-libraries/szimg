use super::utils;
use super::Mode;
use crate::lib::Image;

use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct PBM {
    magic_number: &'static str,
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl PBM {
    pub fn new(width: u32, height: u32, mode: Mode, data: &Vec<u8>) -> Self {
        match mode {
            Mode::Ascii => Self {
                magic_number: "P1",
                width,
                height,
                data: utils::byte_to_char(data),
            },
            Mode::Binary => {
                let mut index: usize = 0;
                let mut converted_data = Vec::new();
                while (index as u32) < height {
                    let row_start = index * (width as usize);
                    let row_end = std::cmp::min(row_start + width as usize, data.len());
                    let line = &data[row_start..row_end].to_vec();
                    // Extra bits are added at the end of each row to fill a whole byte
                    let bytes = utils::u8_to_bits(line);
                    converted_data.push(bytes);
                    index += 1;
                }
                Self {
                    magic_number: "P4",
                    width,
                    height,
                    data: converted_data.iter().cloned().flatten().collect::<Vec<_>>(),
                }
            }
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
