use super::ChecksumIterator;

pub fn calc(bytes: &Vec<u8>) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    for byte in bytes.iter() {
        a = (a + *byte as u32) % 65521;
        b = (b + a) % 65521;
    }

    b << 16 + a
}

pub struct AdlerIterator {
    a: u32,
    b: u32,
}

impl ChecksumIterator for AdlerIterator {
    fn new() -> Self {
        AdlerIterator { a: 1, b: 0 }
    }

    fn iter(&mut self, bytes: &Vec<u8>) {
        for byte in bytes.iter() {
            self.a = (self.a + *byte as u32) % 65521;
            self.b = (self.a + self.b) % 65521;
        }
    }

    fn get(&self) -> u32 {
        (self.b << 16) | self.a
    }
}