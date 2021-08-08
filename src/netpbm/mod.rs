mod pbm;

use pbm::PBM;
use crate::lib::Image;

use std::error::Error;

pub fn save_pbm<const WIDTH: usize, const HEIGHT: usize>(path: &str, data: [[u8; WIDTH]; HEIGHT]) -> Result<(), Box<dyn Error>> {
    let data = data.iter().cloned().flatten().collect::<Vec<u8>>();
    let pbm = PBM::new(WIDTH as u32, HEIGHT as u32, &data);
    pbm.dump(path)
}

#[cfg(test)]
mod tests {
    use super::save_pbm;

    #[test]
    fn test_save_pbm() {
        let bytes: [[u8; 6]; 10] = [
            [48, 48, 48, 48, 49, 48],
            [48, 48, 48, 48, 49, 48],
            [48, 48, 48, 48, 49, 48],
            [48, 48, 48, 48, 49, 48],
            [48, 48, 48, 48, 49, 48],
            [48, 48, 48, 48, 49, 48],
            [49, 48, 48, 48, 49, 48],
            [48, 49, 49, 49, 48, 48],
            [48, 48, 48, 48, 48, 48],
            [48, 48, 48, 48, 48, 48],
        ];
        save_pbm("./image/j.pbm", bytes).unwrap();
    }
}