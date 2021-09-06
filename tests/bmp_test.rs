mod helper;

use szimg::bmp::save_bmp;
use helper::diff_file;

#[test]
fn test_save_png_rgb() {
    let mut bmp_array = [[[0_u8; 3]; 255]; 255];
    for outer_index in 0..255 {
        for inner_index in 0..255 {
            bmp_array[outer_index][inner_index][0] = outer_index as u8;
            bmp_array[outer_index][inner_index][1] = inner_index as u8;
            bmp_array[outer_index][inner_index][2] = 128;
        }
    }
    save_bmp("./tests/output/rgb.bmp", bmp_array).unwrap();

    assert!(diff_file("./tests/output/rgb.bmp", "./tests/templates/rgb.bmp"));
}