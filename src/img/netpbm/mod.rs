mod pbm;
mod pgm;
mod ppm;
mod pam;
mod utils;

use super::{Image, Serializable};
use pbm::PBM;
use pgm::PGM;
use ppm::PPM;
use pam::{PAM, TupleType};

use std::error::Error;

pub enum Mode {
    Ascii,
    Binary,
}

pub fn save_pbm<const WIDTH: usize, const HEIGHT: usize>(
    path: &str,
    data: [[u8; WIDTH]; HEIGHT],
    mode: Mode,
) -> Result<(), Box<dyn Error>> {
    let data = data.iter().cloned().flatten().collect::<Vec<_>>();
    let pbm = PBM::new(WIDTH as u32, HEIGHT as u32, mode, &data);
    pbm.dump(path)
}

pub fn save_pgm<const WIDTH: usize, const HEIGHT: usize>(
    path: &str,
    data: [[u8; WIDTH]; HEIGHT],
    max_value: u8,
    mode: Mode,
) -> Result<(), Box<dyn Error>> {
    let data = data.iter().cloned().flatten().collect::<Vec<_>>();
    let pbm = PGM::new(mode, WIDTH as u32, HEIGHT as u32, max_value, &data);
    pbm.dump(path)
}

pub fn save_ppm<const WIDTH: usize, const HEIGHT: usize>(
    path: &str,
    data: [[[u8; 3]; WIDTH]; HEIGHT],
    max_value: u8,
    mode: Mode,
) -> Result<(), Box<dyn Error>> {
    let data = data.iter().cloned().flatten().flatten().collect::<Vec<_>>();
    let pbm = PPM::new(mode, WIDTH as u32, HEIGHT as u32, max_value, &data);
    pbm.dump(path)
}

pub fn save_pam_2d<const WIDTH: usize, const HEIGHT: usize>(
    path: &str,
    data: [[u8; WIDTH]; HEIGHT],
    mode: TupleType,
) -> Result<(), Box<dyn Error>> {
    assert!(mode != TupleType::RGB && mode != TupleType::RGBAlpha);
    let data = data.iter().cloned().flatten().collect::<Vec<_>>();
    let pbm = PAM::new(mode, WIDTH as u32, HEIGHT as u32, &data);
    pbm.dump(path)
}

// Introduce CHANNEL to support alpha channel
pub fn save_pam_3d<const WIDTH: usize, const HEIGHT: usize, const CHANNEL: usize>(
    path: &str,
    data: [[[u8; CHANNEL]; WIDTH]; HEIGHT],
    mode: TupleType,
) -> Result<(), Box<dyn Error>> {
    assert!(mode == TupleType::RGB || mode == TupleType::RGBAlpha);
    let data = data.iter().cloned().flatten().flatten().collect::<Vec<_>>();
    let pbm = PAM::new(mode, WIDTH as u32, HEIGHT as u32, &data);
    pbm.dump(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_pbm() {
        let bytes = [
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
        save_pbm("./image/j.pbm", bytes, Mode::Binary).unwrap();
    }

    #[test]
    fn test_save_pgm() {
        let bytes = [
            [
                0, 3, 3, 3, 3, 0, 0, 7, 7, 7, 7, 0, 0, 11, 11, 11, 11, 0, 0, 15, 15, 15, 15, 0,
            ],
            [
                0, 3, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 15, 0, 0, 15, 0,
            ],
            [
                0, 3, 3, 3, 0, 0, 0, 7, 7, 7, 0, 0, 0, 11, 11, 11, 0, 0, 0, 15, 15, 15, 15, 0,
            ],
            [
                0, 3, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0,
            ],
            [
                0, 3, 0, 0, 0, 0, 0, 7, 7, 7, 7, 0, 0, 11, 11, 11, 11, 0, 0, 15, 0, 0, 0, 0,
            ],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ];
        save_pgm("./image/feep.pgm", bytes, 15, Mode::Binary).unwrap();
    }

    #[test]
    fn test_save_ppm() {
        let bytes = [
            [[255, 0, 0], [0, 0, 255], [0, 0, 255]],
            [[255, 255, 0], [255, 255, 255], [0, 0, 0]],
        ];

        save_ppm("./image/6_colors.ppm", bytes, 255, Mode::Ascii).unwrap();
    }

    // Unfortunately our OS does not support .pam file
    #[test]
    fn test_save_pam() {
        let bytes = [
            [[0, 0, 255, 255], [0, 255, 0, 255], [255, 0, 0, 255], [255, 255, 255, 255]],
            [[0, 0, 255, 127], [0, 255, 0, 127], [255, 0, 0, 127], [255, 255, 255, 127]],
        ];
        save_pam_3d("./image/8_colors.pam", bytes, TupleType::RGBAlpha).unwrap();
    }
}
