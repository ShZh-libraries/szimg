pub fn get_dct(g: [[i32; 8]; 8]) -> [[f64; 8]; 8] {
    let mut result = [[0.; 8]; 8];    
    // Phase 1
    let mut temp = g;
    for u in 0..8 {
        for v in 0..8 {
            temp[u][v] -= 128;
        }
    }

    // Phase 2
    for u in 0..8 {
        for v in 0..8 {
            let a_u = if u == 0 { 1. / 2_f64.sqrt() } else { 1. };
            let a_v = if v == 0 { 1. / 2_f64.sqrt() } else { 1. };
            result[u][v] = 0.25 * a_u * a_v * multiply_add(&temp, u, v);
        }
    }

    result
}

fn multiply_add(g: &[[i32; 8]; 8], u: usize, v: usize) -> f64 {
    let mut result = 0.;

    for x in 0..8 {
        for y in 0..8 {
            let cos1 = f64::cos((2 * x + 1) as f64 * u as f64 * std::f64::consts::PI / 16.);
            let cos2 = f64::cos((2 * y + 1) as f64 * v as f64 * std::f64::consts::PI / 16.);
            result += g[x][y] as f64 * cos1 * cos2;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::get_dct;

    #[test]
    fn test_dct() {
        let g = [
            [52, 55, 61, 66, 70, 61, 64, 73],
            [63, 59, 55, 90, 109, 85, 69, 72],
            [62, 59, 68, 113, 144, 104, 66, 73],
            [63, 58, 71, 122, 154, 106, 70, 69],
            [67, 61, 68, 104, 126, 88, 68, 70],
            [79, 65, 60, 70, 77, 68, 58, 75],
            [85, 71, 64, 59, 55, 61, 65, 83],
            [87, 79, 69, 68, 65, 76, 78, 94],
        ];

        println!("{:?}", get_dct(g));
    }
}
