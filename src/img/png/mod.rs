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

#[cfg(test)]
mod tests {
    use super::save_png;

    #[test]
    fn test_save_png_without_alpha() {
        // Prepare data
        let mut png_array = [[[0_u8; 3]; 255]; 255];
        for outer_index in 0..255 {
            for inner_index in 0..255 {
                png_array[outer_index][inner_index][0] = outer_index as u8;
                png_array[outer_index][inner_index][1] = inner_index as u8;
                png_array[outer_index][inner_index][2] = 128;
            }
        }
        save_png("./image/rgb.png", png_array).unwrap();
    }

    #[test]
    fn test_save_png_with_alpha() {
        // Prepare data
        let mut png_array = [[[0_u8; 4]; 255]; 255];
        for outer_index in 0..255 {
            for inner_index in 0..255 {
                png_array[outer_index][inner_index][0] = outer_index as u8;
                png_array[outer_index][inner_index][1] = inner_index as u8;
                png_array[outer_index][inner_index][2] = 128;
                png_array[outer_index][inner_index][3] =
                    ((outer_index as u16 + inner_index as u16) / 2) as u8;
            }
        }
        save_png("./image/rgba.png", png_array).unwrap();
    }
}
