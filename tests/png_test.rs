mod helper;

use tiny_img::png::save_png;
use helper::diff_file;

#[test]
fn test_save_png_rgb() {
    let mut png_array = [[[0_u8; 3]; 255]; 255];
    for outer_index in 0..255 {
        for inner_index in 0..255 {
            png_array[outer_index][inner_index][0] = outer_index as u8;
            png_array[outer_index][inner_index][1] = inner_index as u8;
            png_array[outer_index][inner_index][2] = 128;
        }
    }
    save_png("./tests/output/rgb.png", png_array).unwrap();

    assert!(diff_file("./tests/output/rgb.png", "./tests/templates/rgb.png"));
}

#[test]
fn test_save_png_rgba() {
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

    save_png("./tests/output/rgba.png", png_array).unwrap();

    assert!(diff_file("./tests/output/rgba.png", "./tests/templates/rgba.png"));
}