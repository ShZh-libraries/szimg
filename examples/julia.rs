mod complex;

use complex::Complex;
use tiny_img::netpbm::{save_ppm, Mode};

fn scale(x: f64, y: f64, r: f64, width: isize, height: isize) -> (f64, f64) {
    (
        r * (x - width as f64 / 2.) / (width as f64 / 2.),
        r * (y - height as f64 / 2.) / (height as f64 / 2.),
    )
}

const C: Complex = Complex {
    real: -0.8,
    imaginary: 0.156,
};

fn create_julia() -> [[[u8; 3]; 1000]; 1000] {
    let mut res = [[[0; 3]; 1000]; 1000];

    let mut y: usize = 0;
    while y < 1000 {
        let mut x: usize = 0;
        while x < 1000 {
            let (zx, zy) = scale(x as f64, y as f64, 1., 1000, 1000);
            let mut temp = Complex {
                real: zx as f64,
                imaginary: zy as f64,
            };
            // iter
            let mut iter_time = 0;
            while temp.get_length() < 10000. && iter_time < 255 {
                temp = temp * temp + C;
                iter_time += 1;
            }
            res[y][x][0] = if iter_time == 200 { 0 } else { iter_time };
            x += 1;
        }

        y += 1;
    }

    res
}

fn main() {
    let data = create_julia();
    save_ppm("./image/mandlebrot.ppm", data, 255, Mode::Binary).unwrap();
}
