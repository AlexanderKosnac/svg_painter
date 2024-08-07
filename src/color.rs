use rand::Rng;
use crate::util;

pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r: r, g: g, b: b, a: a }
    }

    pub fn new_rand() -> Self {
        let mut rng = rand::thread_rng();
        Rgba::new(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255))
    }

    pub fn new_black() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }

    pub fn as_hex(&self) -> String {
        return format!("#{:0>2X}{:0>2X}{:0>2X}", self.r, self.g, self.b);
    }

    pub fn mutate(&mut self, magnitude: f64) {
        let mut rng = rand::thread_rng();
        let dir_vec: Vec<f64> = (0..4).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        let len = dir_vec.iter().map(|i| i.powf(2.0)).sum::<f64>().sqrt();
        let normed_and_scaled = dir_vec.iter().map(|i| (i/len)*magnitude).collect::<Vec<f64>>();

        self.r = util::bounded_add(self.r, normed_and_scaled[0] as i64);
        self.g = util::bounded_add(self.g, normed_and_scaled[1] as i64);
        self.b = util::bounded_add(self.b, normed_and_scaled[2] as i64);
        self.a = util::bounded_add(self.a, normed_and_scaled[3] as i64);
    }
}

impl Clone for Rgba {
    fn clone(&self) -> Self {
        Rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}
