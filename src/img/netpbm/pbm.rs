use super::utils;
use super::Mode;
use super::{Image, Serializable};

pub struct PBM {
    mode: Mode,
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl PBM {
    pub fn new(width: u32, height: u32, mode: Mode, data: &Vec<u8>) -> Self {
        match mode {
            Mode::Ascii => Self {
                mode,
                width,
                height,
                // Convert each 0b0 and 0b1 to '0' and '1'
                data: utils::byte_to_char(data),
            },
            Mode::Binary => Self {
                mode,
                width,
                height,
                // e.g.: [0b1, 0b0, 0b0, 0b0, 0b0] -> 0b10000000
                data: compress_bits_to_u8_array(width, height, data),
            },
        }
    }
}

// Compress byte to bit sequence
fn compress_bits_to_u8_array(width: u32, height: u32, bits: &Vec<u8>) -> Vec<u8> {
    let mut converted_data = Vec::new();
    for index in 0..height {
        let index = index as usize;
        let row_start = index * (width as usize);
        let row_end = std::cmp::min(row_start + width as usize, bits.len());
        let line = &bits[row_start..row_end].to_vec();
        // Extra bits are added at the end of each row to fill a whole byte
        let bytes = utils::u8_to_bits(line);
        converted_data.push(bytes);
    }

    converted_data.iter().cloned().flatten().collect::<Vec<_>>()
}

impl Serializable for PBM {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let header = format!(
            "{magic_number}\n{width} {height}\n",
            magic_number = if let Mode::Ascii = self.mode {
                "P1"
            } else {
                "P4"
            },
            width = self.width,
            height = self.height,
        );
        bytes.extend(header.bytes());
        bytes.extend(&self.data);

        bytes
    }
}

impl Image for PBM {}
