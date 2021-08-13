mod dct;
mod quant;
mod huffman;
mod rle;
mod jpeg;

use super::{Serializable, Image};
use jpeg::JPEG;

use std::error::Error;

pub fn save_jpg<const WIDTH: usize, const HEIGHT: usize>(
    path: &str,
    data: [[u8; WIDTH]; HEIGHT],
) -> Result<(), Box<dyn Error>> {
    let data = data.iter().cloned().flatten().collect::<Vec<_>>();
    JPEG::new(WIDTH as u16, HEIGHT as u16, &data).dump(path)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_save_jpeg() {
        let data = [
            [52, 55, 61, 66, 70, 61, 64, 73],
            [63, 59, 55, 90, 109, 85, 69, 72],
            [62, 59, 68, 113, 144, 104, 66, 73],
            [63, 58, 71, 122, 154, 106, 70, 69],
            [67, 61, 68, 104, 126, 88, 68, 70],
            [79, 65, 60, 70, 77, 68, 58, 75],
            [85, 71, 64, 59, 55, 61, 65, 83],
            [87, 79, 69, 68, 65, 76, 78, 94],
        ];

        save_jpg("./image/jpeg.jpg", data).unwrap();
    }
}