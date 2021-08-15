mod helper;

use tiny_img::netpbm::{ save_pbm, save_pgm, save_ppm, Mode };
use helper::diff_file;

#[test]
fn test_save_pbm() {
    let data = [
        [0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1, 0],
        [1, 0, 0, 0, 1, 0],
        [0, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
    ];
    save_pbm("./tests/output/j.pbm", data, Mode::Binary).unwrap(); 

    assert!(diff_file("./tests/output/j.pbm", "./tests/templates/j.pbm"));
}

#[test]
fn test_save_pgm() {
    let data = [
        [0, 3, 3, 3, 3, 0, 0, 7, 7, 7, 7, 0, 0, 11, 11, 11, 11, 0, 0, 15, 15, 15, 15, 0,],
        [0, 3, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 15, 0, 0, 15, 0,],
        [0, 3, 3, 3, 0, 0, 0, 7, 7, 7, 0, 0, 0, 11, 11, 11, 0, 0, 0, 15, 15, 15, 15, 0,],
        [0, 3, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0,],
        [0, 3, 0, 0, 0, 0, 0, 7, 7, 7, 7, 0, 0, 11, 11, 11, 11, 0, 0, 15, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
    ];
    save_pgm("./tests/output/feep.pgm", data, 15, Mode::Ascii).unwrap();

    assert!(diff_file("./tests/output/feep.pgm", "./tests/templates/feep.pgm"));
}

#[test]
fn test_save_ppm() {
    let data = [
        [[255, 0, 0], [0, 0, 255], [0, 0, 255]],
        [[255, 255, 0], [255, 255, 255], [0, 0, 0]],
    ];

    save_ppm("./tests/output/6_colors.ppm", data, 255, Mode::Ascii).unwrap();

    assert!(diff_file("./tests/output/6_colors.ppm", "./tests/templates/6_colors.ppm"));
}


// Unfortunately our OS does not support .pam file