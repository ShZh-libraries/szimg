mod bmp;

use bmp::BMP;

use crate::Image;
use std::error::Error;

pub fn save_bmp<const WIDTH: usize, const HEIGHT: usize>(
    path: &str,
    data: [[[u8; 3]; WIDTH]; HEIGHT]
) -> Result<(), Box<dyn Error>> {
    let data = data.iter().cloned().flatten().flatten().collect::<Vec<_>>();
    BMP::new(WIDTH as u32, HEIGHT as u32, &data).dump(path)
}
