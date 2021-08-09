mod png;

use png::PNG;
use crate::lib::Image;

use std::error::Error;

pub fn save_png<const WIDTH: usize, const HEIGHT: usize>(
    path: &str,
    data: [[[u8; 3]; WIDTH]; HEIGHT],
) -> Result<(), Box<dyn Error>> {
    let data = data.iter().cloned().flatten().flatten().collect::<Vec<_>>();
    PNG::new(WIDTH as u32, HEIGHT as u32, false, &data).dump(path)
}

#[cfg(test)]
mod tests {
    use super::save_png;

    #[test]
    fn test_save_png_with_alpha() {
        // Prepare data
        let mut png_array = [[[0_u8; 3]; 255]; 255];
        let mut outer_index: usize = 0;
        while outer_index < 255 {
            let mut inner_index: usize = 0;
            while inner_index < 255 {
                png_array[outer_index][inner_index][0] = outer_index as u8;
                png_array[outer_index][inner_index][1] = inner_index as u8;
                png_array[outer_index][inner_index][2] = 128;

                inner_index += 1;
            }

            outer_index += 1;
        }
        save_png("./image/rgb.png", png_array).unwrap();
    }
}