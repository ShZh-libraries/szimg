mod adler;
mod crc;
mod png;

use super::{Image, Serializable};
use png::PNG;

use std::error::Error;

pub fn save_png<const WIDTH: usize, const HEIGHT: usize, const CHANNEL: usize>(
    path: &str,
    data: [[[u8; CHANNEL]; WIDTH]; HEIGHT],
) -> Result<(), Box<dyn Error>> {
    let data = data.iter().cloned().flatten().flatten().collect::<Vec<_>>();
    let has_alpha = if CHANNEL == 3 { false } else { true };
    PNG::new(WIDTH as u32, HEIGHT as u32, has_alpha, &data).dump(path)
}

pub trait ChecksumIterator {
    fn new() -> Self;

    fn iter(&mut self, bytes: &Vec<u8>);

    fn get(&self) -> u32;
}
