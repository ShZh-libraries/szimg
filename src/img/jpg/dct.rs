/// Keep simple for readablity

pub fn get_dct(g: [i32; 64]) -> [f64; 64] {
    let mut result = [0.; 64];
    // Phase 1
    let mut temp = g;
    for v in 0..8 {
        for u in 0..8 {
            temp[v * 8 + u] -= 128;
        }
    }

    // Phase 2
    for v in 0..8 {
        for u in 0..8 {
            let a_u = if u == 0 { 1. / 2_f64.sqrt() } else { 1. };
            let a_v = if v == 0 { 1. / 2_f64.sqrt() } else { 1. };
            result[v * 8 + u] = 0.25 * a_u * a_v * multiply_add(&temp, u, v);
        }
    }

    result
}

fn multiply_add(g: &[i32; 64], u: usize, v: usize) -> f64 {
    let mut result = 0.;

    for y in 0..8 {
        for x in 0..8 {
            let cos2 = f64::cos((2 * x + 1) as f64 * u as f64 * std::f64::consts::PI / 16.);
            let cos1 = f64::cos((2 * y + 1) as f64 * v as f64 * std::f64::consts::PI / 16.);
            result += g[y * 8 + x] as f64 * cos1 * cos2;
        }
    }

    result
}
