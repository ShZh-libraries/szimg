use rand::Rng;
use tiny_img::netpbm::{save_ppm, Mode};

fn iter(rng: &mut rand::prelude::ThreadRng, (x, y): (f32, f32)) -> (f32, f32) {
    match rng.gen_range(0..101) {
        1 => (0., 0.16 * y),
        2..=86 => (0.85 * x + 0.04 * y, -0.04 * x + 0.85 * y + 1.6),
        87..=93 => (0.2 * x - 0.26 * y, 0.23 * x + 0.22 * y + 1.6),
        _ => (-0.15 * x + 0.28 * y, 0.26 * x + 0.24 * y + 0.44)
    }
} 

fn create_barnsley_fern() -> [[[u8; 3]; 600]; 600] {
    let mut data = [[[0; 3]; 600]; 600];
    let mut rng = rand::thread_rng();

    let (mut x, mut y) = (0., 0.);
    for _ in 0..100000 {
        if x > 3. {
            println!("{} {}", x, y);
        }
        let plot_x = 50. * x + 300.;
        let plot_y = 600. - (50. * y + 50.);
        data[plot_y as usize][plot_x as usize][1] = 255;

        let tuple = iter(&mut rng, (x, y));
        x = tuple.0;
        y = tuple.1;
    }

    data
}

fn main() {
    let data = create_barnsley_fern();
    save_ppm("./image/barnsley_fern.ppm", data, 255, Mode::Binary).unwrap();
}