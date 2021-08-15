mod dct;
mod huffman;
mod jpeg;
mod quant;
mod rle;

use super::{Image, Serializable};
use jpeg::JPEG;

use std::error::Error;

pub fn save_jpg<const WIDTH: usize, const HEIGHT: usize>(
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

    #[test]
    fn test_save_rgb_jpeg() {
        let mut data = [[[0; 3]; 8]; 8];
        let prepared_data = [
            [52, 55, 61, 66, 70, 61, 64, 73],
            [63, 59, 55, 90, 109, 85, 69, 72],
            [62, 59, 68, 113, 144, 104, 66, 73],
            [63, 58, 71, 122, 154, 106, 70, 69],
            [67, 61, 68, 104, 126, 88, 68, 70],
            [79, 65, 60, 70, 77, 68, 58, 75],
            [85, 71, 64, 59, 55, 61, 65, 83],
            [87, 79, 69, 68, 65, 76, 78, 94],
        ];
        for y in 0..8 {
            for x in 0..8 {
                data[y][x] = [prepared_data[y][x]; 3];
            }
        }
        
        save_jpg_rgb("./image/jpeg2.jpg", data).unwrap();
    }
}
