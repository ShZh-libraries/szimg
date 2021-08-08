mod pbm;
mod pgm;
mod utils;

use pbm::PBM;
use pgm::PGM;
use crate::lib::Image;

use std::error::Error;

pub fn save_pbm<const WIDTH: usize, const HEIGHT: usize>(path: &str, data: [[u8; WIDTH]; HEIGHT]) -> Result<(), Box<dyn Error>> {
    let data = data.iter().flatten().map(|x| x + 48).collect::<Vec<u8>>();
    let pbm = PBM::new(WIDTH as u32, HEIGHT as u32, &data);
    pbm.dump(path)
}

pub fn save_pgm<const WIDTH: usize, const HEIGHT: usize>(path: &str, data: [[u8; WIDTH]; HEIGHT], max_value: u32) -> Result<(), Box<dyn Error>> {
    let data = data.iter().cloned().flatten().collect::<Vec<u8>>();
    let pbm = PGM::new(WIDTH as u32, HEIGHT as u32, max_value, &data);
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
        save_pbm("./image/j.pbm", bytes).unwrap();
    }

    #[test]
    fn test_save_pgm() {
        let bytes = [
            [0, 3, 3, 3, 3, 0, 0, 7, 7, 7, 7, 0, 0, 11, 11, 11, 11,  0,  0, 15, 15, 15, 15,  0,],
            [0, 3, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 11,  0,  0,  0,  0,  0, 15,  0,  0, 15,  0,],
            [0, 3, 3, 3, 0, 0, 0, 7, 7, 7, 0, 0, 0, 11, 11, 11,  0,  0,  0, 15, 15, 15, 15,  0,],
            [0, 3, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 11,  0,  0,  0,  0,  0, 15,  0,  0,  0,  0,],
            [0, 3, 0, 0, 0, 0, 0, 7, 7, 7, 7, 0, 0, 11, 11, 11, 11,  0,  0, 15,  0,  0,  0,  0,],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,],
        ];
        save_pgm("./image/feep.pgm", bytes, 15).unwrap();
    }
}