use noise::*;

pub struct Generator {
    height: f64,
    zero: Constant,
    fractal: Fbm,
    cave: RidgedMulti
}

impl Generator {
    pub fn new(seed: u32, height: f64) -> Self {
        let zero = Constant::new(0.0);

        let mut fractal = Fbm::new().set_seed(seed);
        fractal.frequency = 0.005;

        let mut cave_fractal = RidgedMulti::new().set_seed(seed);
        cave_fractal.octaves = 1;
        cave_fractal.frequency = 0.01;

        Self {
            zero: zero,
            fractal: fractal,
            cave: cave_fractal,
            height: height
        }
    }
}

impl NoiseFn<[f64;2]> for Generator {
    fn get(&self, point: [f64; 2]) -> f64 {
        let scaled_fractal = ScaleBias::new(&self.fractal).set_scale(50.0);
        let scaled_fractal2 = TranslatePoint::new(&scaled_fractal).set_x_translation(100.0).set_y_translation(100.0);

        let without_overhangs_fractal = ScalePoint::new(&scaled_fractal).set_y_scale(0.8);
        let gradient = Ground::new(self.height, 0.0);
        let ground = Displace::new(&gradient, self.zero, without_overhangs_fractal, self.zero, self.zero);
        //return gradient.get(point);

        let caves = Displace::new(&self.cave, &scaled_fractal, scaled_fractal2, self.zero, self.zero);
        let one = Constant::new(1.0);
        let mut small_caves = Select::new(&self.zero, &one, &caves);
        small_caves.bounds = (0.0+gradient.get(point), 1.0);
        let not_inverted_caves = Min::new(&caves, &scaled_fractal);
        let deep_caves = Invert::new(&not_inverted_caves);
        let noise = Select::new(&self.zero, &deep_caves, &ground);
        noise.get(point)
    }
}

pub struct Ground {
    height: f64,
    middle: f64
}

impl Ground {
    pub fn new(height: f64, middle: f64) -> Self {
        Self {
            height: height,
            middle: middle
        }
    }
}

impl NoiseFn<[f64; 2]> for Ground {
    fn get(&self, point: [f64; 2]) -> f64 {
        point[1] / self.height * 2.0 - 1.0
    }
}

pub struct Depth {
    limit: f64
}

impl Depth {
    pub fn new(limit: f64) -> Self {
        Self {
            limit: limit
        }
    }
}

impl NoiseFn<[f64; 2]> for Depth {
    fn get(&self, point: [f64; 2]) -> f64 {
        if point[1] > self.limit {1.0} else {-1.0}
    }
}
