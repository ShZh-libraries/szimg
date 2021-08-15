mod common;
mod dct;
mod quant;
mod huffman;
mod rle;
mod jpeg;

use super::{Image, Serializable};
use jpeg::JPEG;

use std::error::Error;

pub fn save_jpg_gray<const WIDTH: usize, const HEIGHT: usize>(
    path: &str,
    data: [[u8; WIDTH]; HEIGHT],
) -> Result<(), Box<dyn Error>> {
    let data = data.iter().cloned().flatten().collect::<Vec<_>>();
    JPEG::new(WIDTH as u16, HEIGHT as u16, 1, &data).dump(path)
}

pub fn save_jpg_rgb<const WIDTH: usize, const HEIGHT: usize>(
    path: &str,
    data: [[[u8; 3]; WIDTH]; HEIGHT],
) -> Result<(), Box<dyn Error>> {
    // Convert RGB to RGBA
    let data = to_rgba(data).iter().cloned().flatten().flatten().collect::<Vec<_>>();
    JPEG::new(WIDTH as u16, HEIGHT as u16, 3, &data).dump(path)
}

// JPEG does not support alpha channel
// The value of this channel will be automaticlly ignored
pub fn save_jpg_rgba<const WIDTH: usize, const HEIGHT: usize>(
    path: &str,
    data: [[[u8; 4]; WIDTH]; HEIGHT],
) -> Result<(), Box<dyn Error>> {
    let data = data.iter().cloned().flatten().flatten().collect::<Vec<_>>();
    JPEG::new(WIDTH as u16, HEIGHT as u16, 3, &data).dump(path)
}

fn to_rgba<const WIDTH: usize, const HEIGHT: usize>(
    data: [[[u8; 3]; WIDTH]; HEIGHT],
) -> [[[u8; 4]; WIDTH]; HEIGHT] {
    let mut converted = [[[0; 4]; WIDTH]; HEIGHT];
    for (y, row) in data.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            converted[y][x][0] = pixel[0];
            converted[y][x][1] = pixel[1];
            converted[y][x][2] = pixel[2];
            converted[y][x][3] = 255;
        }
    }

    converted
}
