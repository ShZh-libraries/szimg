use rand::Rng;
use tiny_img::netpbm::{save_ppm, Mode};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const ITER_TIME: usize = 100000;

// See https://en.wikipedia.org/wiki/Barnsley_fern for more details
fn iter(rng: &mut rand::prelude::ThreadRng, (x, y): &(f32, f32)) -> (f32, f32) {
    match rng.gen_range(0..=100) {
        1 => (0., 0.16 * y),
        2..=86 => (0.85 * x + 0.04 * y, -0.04 * x + 0.85 * y + 1.6),
        87..=93 => (0.2 * x - 0.26 * y, 0.23 * x + 0.22 * y + 1.6),
        _ => (-0.15 * x + 0.28 * y, 0.26 * x + 0.24 * y + 0.44),
    }
}

fn create_barnsley_fern() -> [[[u8; 3]; WIDTH]; HEIGHT] {
    let mut data = [[[0; 3]; WIDTH]; HEIGHT];
    let mut rng = rand::thread_rng();

    let mut pair = (0., 0.);
    for _ in 0..ITER_TIME {
        // Map to the payload array
        let pos = (
            80. * pair.0 + WIDTH as f32 / 2.,
            HEIGHT as f32 - (50. * pair.1 + 50.),
        );
        data[pos.1 as usize][pos.0 as usize][1] = 255;

        pair = iter(&mut rng, &pair);
    }

    data
}

fn main() {
    let data = create_barnsley_fern();
    save_ppm("./image/barnsley_fern.ppm", data, 255, Mode::Binary).unwrap();
}
