mod helper;

use tiny_img::jpg::{ save_jpg_gray, save_jpg_rgb };
use helper::diff_file;

#[test]
fn test_save_jpg_gray() {
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
    save_jpg_gray("./tests/output/gray.jpg", data).unwrap();

    assert!(diff_file("./tests/output/gray.jpg", "./tests/templates/gray.jpg"));
}

#[test]
fn test_save_jpg_rgb() {
    let mut data = [[[0_u8; 3]; 255]; 255];
    for outer_index in 0..255 {
        for inner_index in 0..255 {
            data[outer_index][inner_index][0] = outer_index as u8;
            data[outer_index][inner_index][1] = inner_index as u8;
            data[outer_index][inner_index][2] = 128;
        }
    }
    save_jpg_rgb("./tests/output/rgb.jpg", data).unwrap();

    assert!(diff_file("./tests/output/rgb.jpg", "./tests/templates/rgb.jpg"));
}

// JPEG does not support alpha channel
// Think about how to convert to YCbCr when encontering transparent