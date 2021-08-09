pub mod adler;
pub mod crc;

pub trait ChecksumIterator {
    fn new() -> Self;

    fn iter(&mut self, bytes: &Vec<u8>);

    fn get(&self) -> u32;
}