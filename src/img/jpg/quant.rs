use lazy_static::lazy_static;

use super::jpeg::Mode;

const QUANT_TABLE_WIDTH: usize = 8;
const QUANT_TABLE_HEIGHT: usize = 8;
const QUANT_TABLE_SIZE: usize = QUANT_TABLE_WIDTH * QUANT_TABLE_HEIGHT;

type QuantTable = [[u8; QUANT_TABLE_WIDTH]; QUANT_TABLE_HEIGHT];

lazy_static! {
    pub static ref LUMINANCE_QUANT_TABLE: QuantTable = {
        let mut qtable = [
            [16, 11, 12, 14, 12, 10, 16, 14],
            [13, 14, 18, 17, 16, 19, 24, 40],
            [26, 24, 22, 22, 24, 49, 35, 37],
            [29, 40, 58, 51, 61, 60, 57, 51],
            [56, 55, 64, 72, 92, 78, 64, 68],
            [87, 69, 55, 56, 80, 109, 81, 87],
            [95, 98, 103, 104, 103, 62, 77, 113],
            [121, 112, 100, 120, 92, 101, 103, 99],
        ];
        prescale_quant_table(&mut qtable);

        qtable
    };
    pub static ref CHROMINANCE_QUANT_TABLE: QuantTable = {
        let mut qtable = [
            [17, 18, 18, 24, 21, 24, 47, 26],
            [26, 47, 99, 66, 56, 66, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
        ];
        prescale_quant_table(&mut qtable);

        qtable
    };
}

// Adjust parameter yourself
fn prescale_quant_table(quant_table: &mut QuantTable) {
    for y in 0..QUANT_TABLE_HEIGHT {
        for x in 0..QUANT_TABLE_WIDTH {
            quant_table[y][x] = (quant_table[y][x] + 1) / 2;
        }
    }
}

pub fn quant(g: [f64; QUANT_TABLE_SIZE], mode: Mode) -> ([i32; QUANT_TABLE_SIZE], i32) {
    let quanted = match mode {
        Mode::Luminance => quant_intern(g, &LUMINANCE_QUANT_TABLE),
        Mode::Chromiance => quant_intern(g, &CHROMINANCE_QUANT_TABLE),
    };

    // Return DC value
    (quanted, quanted[0])
}

fn quant_intern(g: [f64; QUANT_TABLE_SIZE], quant_table: &QuantTable) -> [i32; QUANT_TABLE_SIZE] {
    let mut result = [0_i32; QUANT_TABLE_SIZE];
    for y in 0..QUANT_TABLE_HEIGHT {
        for x in 0..QUANT_TABLE_WIDTH {
            result[y * QUANT_TABLE_WIDTH + x] = (g[y * QUANT_TABLE_WIDTH + x] as f64
                / (LUMINANCE_QUANT_TABLE[y][x] as i32) as f64)
                .round() as i32;
        }
    }

    result
}
