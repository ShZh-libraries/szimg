use lazy_static::lazy_static;

lazy_static! {
    pub static ref QUANT_TABLE: [[[u8; 8]; 8]; 2] = {
        let mut q: [[[u8; 8]; 8]; 2] = [
            // Luminance
            [
                [16, 11, 12, 14, 12, 10, 16, 14],
                [13, 14, 18, 17, 16, 19, 24, 40],
                [26, 24, 22, 22, 24, 49, 35, 37],
                [29, 40, 58, 51, 61, 60, 57, 51],
                [56, 55, 64, 72, 92, 78, 64, 68],
                [87, 69, 55, 56, 80, 109, 81, 87],
                [95, 98, 103, 104, 103, 62, 77, 113],
                [121, 112, 100, 120, 92, 101, 103, 99],
            ],
            // Chromiance
            [
                [17, 18, 18, 24, 21, 24, 47, 26],
                [26, 47, 99, 66, 56, 66, 99, 99],
                [99, 99, 99, 99, 99, 99, 99, 99],
                [99, 99, 99, 99, 99, 99, 99, 99],
                [99, 99, 99, 99, 99, 99, 99, 99],
                [99, 99, 99, 99, 99, 99, 99, 99],
                [99, 99, 99, 99, 99, 99, 99, 99],
                [99, 99, 99, 99, 99, 99, 99, 99],
            ]
        ];

        for i in 0..2 {
            for j in 0..8 {
                for k in 0..8 {
                    q[i][j][k] = (q[i][j][k] + 1) / 2;
                }
            }
        }

        q
    };
}

pub fn quant(g: [f64; 64]) -> ([i32; 64], i32) {
    let mut result = [0_i32; 64];
    for j in 0..8 {
        for k in 0..8 {
            result[j * 8 + k] =
                (g[j * 8 + k] as f64 / (QUANT_TABLE[0][j][k] as i32) as f64).round() as i32;
        }
    }

    // Return DC value
    (result, result[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quant_table() {
        for i in 0..2 {
            for j in 0..8 {
                for k in 0..8 {
                    println!("{}", QUANT_TABLE[i][j][k]);
                }
            }
        }
    }
}
