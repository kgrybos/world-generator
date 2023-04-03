use minifb::{Key, Window, WindowOptions};
use noise::NoiseFn;
use math::round;
use rand::random;

mod noise_functions;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() {
    let generator = noise_functions::Generator::new(random(), HEIGHT as f64);
    let buffer: Vec<u32> = (0..WIDTH*HEIGHT).map(|i| {
        let x = i % WIDTH;
        let y = round::floor(i as f64 / WIDTH as f64, 0);
        generator.get([x as f64, y])
    })
    .map(|item| {
        from_u8_gray(if item > 0.0 {255} else {0})
        //from_u8_gray(((item + 1.0)/2.0*255.0) as u8)
    }).collect();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

//Displace<RidgedMulti, ScaleBias<'static, NoiseFn<Fbm>>, TranslatePoint<&'static ScaleBias<'static, NoiseFn<Fbm>>>, Constant, Constant>
// fn generate(seed: u32) -> impl NoiseFn<[f64;2]> {
//     let zero = Constant::new(0.0);
//     let mut fractal = Fbm::new().set_seed(seed);
//     fractal.frequency = 0.005;
//     let scaled_fractal = ScaleBias::new(&fractal).set_scale(50.0);
//     //let scaled_fractal2 = TranslatePoint::new(&scaled_fractal).set_x_translation(100.0).set_y_translation(100.0);
//     // let without_overhangs_fractal = ScalePoint::new(scaled_fractal).set_y_scale(0.8);
//     // Displace::new(Gradient::new(HEIGHT as f64), zero, without_overhangs_fractal, zero, zero)
//
//     let mut cave_fractal = RidgedMulti::new().set_seed(seed);
//     cave_fractal.octaves = 1;
//     cave_fractal.frequency = 0.01;
//     Displace::new(cave_fractal, &scaled_fractal, &scaled_fractal, zero, zero)
// }

fn from_u8_gray(gray: u8) -> u32 {
    let gray = gray as u32;
    (gray << 16) | (gray << 8) | gray
}
