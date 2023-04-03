#![feature(test)]

extern crate test;

#[cfg(test)]
mod noise_functions;

#[cfg(test)]
mod tests {
    use crate::noise_functions;

    use test::Bencher;
    use rand::random;
    use math::round;
    use noise::NoiseFn;

    #[bench]
    fn setup_generator(b: &mut Bencher) {
        b.iter(|| {
            noise_functions::Generator::new(random(), 10.0)
        });
    }

    #[bench]
    fn generate_chunk(b: &mut Bencher) {
        fn from_u8_gray(gray: u8) -> u32 {
            let gray = gray as u32;
            (gray << 16) | (gray << 8) | gray
        }
        let generator = noise_functions::Generator::new(random(), 10.0);
        b.iter(|| -> Vec<u32> {
            (0..64*64).map(|i| {
                let x = i % 64;
                let y = round::floor(i as f64 / 64 as f64, 0);
                generator.get([x as f64, y])
            }).map(|item| {
                from_u8_gray(if item > 0.0 {255} else {0})
            }).collect()
        });
    }
}
