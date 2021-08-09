mod complex;

use complex::Complex;
use tiny_img::netpbm::{save_ppm, Mode};

fn iter(c: Complex) -> u8 {
    let mut z = c.clone();

    let mut iter_time = 1;
    while iter_time < 50 && (z * z.conjugate()).real < 4.0 {
        z = z * z + c;
        iter_time += 1;
    }

    iter_time
}

fn create_mandlebrot() -> [[[u8; 3]; 1000]; 1000] {
    let mut res = [[[0; 3]; 1000]; 1000];

    let mx = 2.48 / 999.0;
    let my = 2.26 / 999.0;

    let mut x = 0;
    while x < 1000 {
        let mut y = 0;
        while y < 1000 {
            let temp = Complex {
                real: mx * x as f64 - 2.0,
                imaginary: my * y as f64 - 1.13,
            };
            let iter_time = iter(temp);
            res[y][x][2] = iter_time * 5;

            y += 1;
        }
        x += 1;
    }

    res
}

fn main() {
    let data = create_mandlebrot();
    save_ppm("./image/mandlebrot.ppm", data, 255, Mode::Binary).unwrap();
}
