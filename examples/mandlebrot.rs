mod complex;

use complex::Complex;
use szimg::netpbm::{save_ppm, Mode};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

const SCALED_X: f64 = 2.48 / 999.;
const SCALED_Y: f64 = 2.26 / 999.;
const OFFSET_X: f64 = -2.;
const OFFSET_Y: f64 = -1.13;

const MAX_ITER_TIME: u8 = 50;
const THRESHOLD: f64 = 4.;

// f(z) = z ^ 2 + c
fn iter(c: Complex) -> u8 {
    let mut z = c;
    let mut iter_time = 1;
    while iter_time < MAX_ITER_TIME && z.get_length() < THRESHOLD {
        z = z * z + c;
        iter_time += 1;
    }

    iter_time
}

// See https://en.wikipedia.org/wiki/Mandelbrot_set for more details
fn create_mandlebrot() -> [[[u8; 3]; WIDTH]; HEIGHT] {
    let mut res = [[[0; 3]; WIDTH]; HEIGHT];

    // Cache friendly loop
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let temp = Complex {
                real: SCALED_X * x as f64 + OFFSET_X,
                imaginary: SCALED_Y * y as f64 + OFFSET_Y,
            };
            let iter_time = iter(temp);
            // Magic number 5 is to map [0, 50] to [0, 250](almost 255)
            // That's to reduce CPU's budern
            res[y][x][2] = iter_time * 5;
        }
    }

    res
}

fn main() {
    let data = create_mandlebrot();
    save_ppm("./image/mandlebrot.ppm", data, 255, Mode::Binary).unwrap();
}
