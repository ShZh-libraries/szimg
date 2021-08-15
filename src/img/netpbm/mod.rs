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
