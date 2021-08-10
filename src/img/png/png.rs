use super::{ Image, Serializable };

use std::fs::File;
use std::io::Write;
use std::error::Error;

use crate::checksum::{ adler::AdlerIterator, crc, ChecksumIterator };

pub struct PNG {
    magic_number: [u8; 8],
    ihdr_chunk: Chunk<IHDR>,
    idat_chunk: Chunk<IDAT>,
    iend_chunk: Chunk<IEND>,
}

struct Chunk<T: Serializable> {
    length: u32,
    name: [u8; 4],
    payload: T,
    // crc: u32,
}

struct IHDR {
    width: u32,
    height: u32,
    depth: u8,
    alpha: u8,
    compression: u8,
    filter: u8,
    interlace: u8,
}

struct IDAT {
    compression_prefix: [u8; 2],
    data_blocks: Vec<DataBlock>,
    // adler: u32,
}

struct DataBlock {
    is_last: bool,
    block_size: u16,
    filter_prefix: [u8; 1],
    data: Vec<u8>,
}

struct IEND {}

impl PNG {
    pub fn new(width: u32, height: u32, alpha: bool, data: &Vec<u8>) -> Self {
        let idat_length = if alpha {
            2 + height * (5 + width * 4 + 1) + 4
        } else {
            2 + height * (5 + width * 3 + 1) + 4
        };

        Self {
            magic_number: [b'\x89', b'P', b'N', b'G', b'\r', b'\n', b'\x1a', b'\n'],
            ihdr_chunk: Chunk {
                length: 13,
                name: *b"IHDR",
                payload: IHDR::new(width, height, alpha),
            },
            idat_chunk: Chunk {
                length: idat_length,
                name: *b"IDAT",
                payload: IDAT::new(width, height, alpha, data),
            },
            iend_chunk: Chunk {
                length: 0,
                name: *b"IEND",
                payload: IEND {},
            },
        }
    }
}

impl Serializable for PNG {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.magic_number);
        bytes.extend(self.ihdr_chunk.get_bytes());
        bytes.extend(self.idat_chunk.get_bytes());
        bytes.extend(self.iend_chunk.get_bytes());

        bytes
    }
}

impl Image for PNG {
    fn dump(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let bytes = self.get_bytes();
        let mut file = File::create(path)?;
        file.write(&bytes)?;
        Ok(())
    }
}

impl<T> Serializable for Chunk<T>
where
    T: Serializable,
{
    fn get_bytes(&self) -> Vec<u8> {
        let mut crc_bytes = Vec::new();
        crc_bytes.extend(self.name);
        crc_bytes.extend(self.payload.get_bytes());
        // Calculate CRC checksum
        let crc = crc::calc(&crc_bytes).to_be_bytes();

        [&self.length.to_be_bytes(), &crc_bytes[..], &crc].concat()
    }
}

impl IHDR {
    fn new(width: u32, height: u32, alpha: bool) -> Self {
        Self {
            width,
            height,
            depth: 8,
            alpha: if alpha { 6 } else { 2 },
            compression: b'\0',
            filter: b'\0',
            interlace: b'\0',
        }
    }
}

impl Serializable for IHDR {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(13);

        // 32 bit for big-endian
        bytes.extend(self.width.to_be_bytes());
        bytes.extend(self.height.to_be_bytes());
        // others for little-endian
        bytes.extend(self.depth.to_le_bytes());
        bytes.extend(self.alpha.to_le_bytes());
        bytes.extend(self.compression.to_le_bytes());
        bytes.extend(self.filter.to_le_bytes());
        bytes.extend(self.interlace.to_le_bytes());

        bytes
    }
}

impl IDAT {
    fn new(width: u32, height: u32, alpha: bool, data: &Vec<u8>) -> Self {
        // Get number and size of data block
        let block_num = height;
        let block_size: u16 = if alpha {
            (width * 4) as u16
        } else {
            (width * 3) as u16
        };
        // Build data blocks
        let mut data_blocks: Vec<DataBlock> = Vec::with_capacity(block_num as usize);
        for i in 0..block_num {
            let start_pos = (i * block_size as u32) as usize;
            let end_pos = ((i + 1) * block_size as u32) as usize;
            if i == block_num - 1 {
                data_blocks.push(DataBlock {
                    is_last: true,
                    block_size,
                    filter_prefix: *b"\0",
                    data: data[start_pos..end_pos].iter().cloned().collect(),
                });
            } else {
                data_blocks.push(DataBlock {
                    is_last: false,
                    block_size,
                    filter_prefix: *b"\0",
                    data: data[start_pos..end_pos].iter().cloned().collect(),
                });
            }
        }

        Self {
            compression_prefix: *b"\x78\x01",
            data_blocks,
        }
    }
}

impl Serializable for IDAT {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // Compression prefix
        bytes.extend(self.compression_prefix);
        // Data block and calculate adler checksum
        let mut adler_iterator = AdlerIterator::new();
        for data_block in self.data_blocks.iter() {
            let data_block_bytes = data_block.get_bytes_with_adler(&mut adler_iterator);
            bytes.extend(data_block_bytes);
        }
        // Adler checksum
        bytes.extend(adler_iterator.get().to_be_bytes());

        bytes
    }
}

impl DataBlock {
    // The adler checksum is from the part of data block
    fn get_bytes_with_adler(&self, adler_iterator: &mut AdlerIterator) -> Vec<u8> {
        let mut bytes = Vec::with_capacity((6 + self.block_size) as usize);

        bytes.push(if self.is_last { b'\x01' } else { b'\x00' });
        bytes.extend((self.block_size + 1).to_le_bytes());
        bytes.extend((!(self.block_size + 1)).to_le_bytes());
        // Prepare the adler bytes and calculate the checksum
        let mut adler_bytes = Vec::with_capacity((1 + self.block_size) as usize);
        adler_bytes.extend(&self.filter_prefix);
        adler_bytes.extend(&self.data);
        adler_iterator.iter(&adler_bytes);
        bytes.extend(adler_bytes);

        bytes
    }
}

impl Serializable for IEND {
    fn get_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}