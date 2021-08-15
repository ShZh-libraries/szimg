use super::ChecksumIterator;

const CRC_TABLE: [u32; 16] = [
    0, 0x1db71064, 0x3b6e20c8, 0x26d930ac, 0x76dc4190, 0x6b6b51f4, 0x4db26158, 0x5005713c,
    0xedb88320, 0xf00f9344, 0xd6d6a3e8, 0xcb61b38c, 0x9b64c2b0, 0x86d3d2d4, 0xa00ae278, 0xbdbdf21c,
];

pub fn calc(bytes: &Vec<u8>) -> u32 {
    let mut crc: u32 = !0;
    for byte in bytes.iter() {
        crc ^= *byte as u32;
        crc = (crc >> 4) ^ CRC_TABLE[(crc & 15) as usize];
        crc = (crc >> 4) ^ CRC_TABLE[(crc & 15) as usize];
    }

    !crc
}

pub struct CRCIterator {
    crc: u32,
}

impl ChecksumIterator for CRCIterator {
    fn new() -> Self {
        Self { crc: !0 }
    }

    fn iter(&mut self, bytes: &Vec<u8>) {
        for byte in bytes.iter() {
            self.crc ^= *byte as u32;
            self.crc = (self.crc >> 4) ^ CRC_TABLE[(self.crc & 15) as usize];
            self.crc = (self.crc >> 4) ^ CRC_TABLE[(self.crc & 15) as usize];
        }
    }

    fn get(&self) -> u32 {
        !self.crc
    }
}