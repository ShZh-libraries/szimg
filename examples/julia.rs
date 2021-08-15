mod complex;

use complex::Complex;
use szimg::netpbm::{save_ppm, Mode};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const SCALED_FACTOR: f64 = 1.;
const MAX_ITER_TIME: u8 = 255;
const THRESHOLD: f64 = 10000.;

const C: Complex = Complex {
    real: -0.8,
    imaginary: 0.156,
};

// Scale from [0, WIDTH] and [0, HEIGHT] to [-r, r]
fn scale(x: f64, y: f64, r: f64, width: isize, height: isize) -> (f64, f64) {
    (
        r * (x - width as f64 / 2.) / (width as f64 / 2.),
        r * (y - height as f64 / 2.) / (height as f64 / 2.),
    )
}

// f(z) = z ^ 2 + c
fn iter(init: Complex) -> u8 {
    let mut zc = init;
    let mut iter_time = 0;
    while iter_time < MAX_ITER_TIME && zc.get_length() < THRESHOLD {
        zc = zc * zc + C;
        iter_time += 1;
    }

    iter_time
}

// See https://en.wikipedia.org/wiki/Julia_set for more details
fn create_julia() -> [[[u8; 3]; WIDTH]; HEIGHT] {
    let mut res = [[[0; 3]; WIDTH]; HEIGHT];

    // Cache friendly loop
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let (zx, zy) = scale(
                x as f64,
                y as f64,
                SCALED_FACTOR,
                WIDTH as isize,
                HEIGHT as isize,
            );
            let zc = Complex {
                real: zx as f64,
                imaginary: zy as f64,
            };
            let iter_time = iter(zc);
            res[y][x][0] = if iter_time == 200 { 0 } else { iter_time };
        }
    }

    res
}

fn main() {
    let data = create_julia();
    save_ppm("./image/julia_set.ppm", data, 255, Mode::Binary).unwrap();
}
