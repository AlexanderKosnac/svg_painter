use rand::Rng;

use crate::Controller;
use crate::color::Rgba;

pub static STROKE_DIMENSION: (f32, f32) = (100.0, 100.0);
pub static STROKES: [&str; 4] = [
    "<path id=\"stroke-0\" d=\"M 40.338582,26.098515 C 38.053844,36.092479 30.282228,33.261594 21.634084,38.835496 12.427584,40.500523 2.3748283,50.452193 -6.2913621,45.484871 -14.314653,48.44545 -24.388187,39.949125 -31.395597,35.173134 -39.806566,34.932778 -45.214489,20.982536 -46.188418,10.768909 c -3.9224,-9.80799118 -3.235775,-23.424642 2.270978,-29.080333 4.582211,-1.792749 3.117749,-10.611915 6.824023,-14.965833 11.666209,-3.93886 17.440251,-13.191695 29.9603175,-13.980471 12.0631684,0.645712 25.3505425,0.142897 34.9983975,8.752781 7.210887,8.511997 16.572265,16.270243 18.263217,28.086858 1.10511,9.0539519 -3.025009,10.4871012 -2.327784,19.9431268 4.485726,6.3695612 -0.781874,4.8493372 -2.474996,12.2554412 z\" />",
    "<path id=\"stroke-1\" d=\"M 45.723817,2.8912642 C 54.475195,10.70549 50.327586,14.055511 45.217729,22.108315 47.667962,29.23073 32.325404,35.530249 27.907317,39.786018 21.269965,43.84541 14.74429,48.68999 6.5967441,48.832574 -3.4612464,50.779215 -12.797589,46.111222 -22.224423,43.474083 -33.331842,44.006991 -40.087321,34.0926 -42.303249,24.32579 c -2.39406,-7.620345 -1.738816,-16.2812556 -7.052962,-22.6836803 -3.275455,-5.8581219 -4.620985,-15.2285827 3.827301,-17.0873347 9.368082,-3.641724 3.348983,-13.821374 7.320216,-20.404299 4.194818,-6.411198 12.650411,-7.38745 18.945493,-11.094932 11.0724129,-3.117367 22.8493623,-2.587909 34.145835,-1.028339 5.810758,4.088285 20.56037,2.52364 18.177167,11.60148 2.677228,4.423485 11.566197,3.33694 10.068583,11.633193 1.237229,7.814004 1.982478,3.869118 4.497359,11.668166 -1.472752,4.6398545 -0.411552,7.8719794 -1.901926,15.9612202 z\" />",
    "<path id=\"stroke-2\" d=\"m 46.273,1.30885 c -3.860615,8.3253396 0.521391,18.873147 -5.837843,26.286688 -4.668268,1.703879 -5.262114,6.766617 -6.082802,10.911437 -2.257006,6.938515 -7.22269,14.403192 -15.118071,15.144581 -5.88858,-1.117243 -9.8470264,-9.656841 -16.5660175,-4.574979 -6.0969877,3.891459 -11.8702633,0.22161 -17.5187425,-1.805673 -6.484488,0.513073 -10.206198,-4.906001 -13.268046,-9.70538 -3.799386,-6.060567 -8.017309,-12.756845 -15.310325,-14.894563 -8.632973,-4.460615 -10.602104,-15.7453243 -8.253595,-24.4237471 0.873634,-5.6613268 5.342285,-9.6943429 6.138835,-15.2975549 4.10579,-12.145954 15.254418,-20.995429 17.139784,-34.035302 8.263848,-1.674641 15.624189,6.55959 23.8493347,3.213568 7.0020878,-5.240603 16.4137323,-1.881698 23.3197233,1.786365 7.336762,3.897887 13.905695,9.37322 21.900452,11.913061 8.291635,4.189622 10.673044,14.697181 9.047238,23.1968 -1.709911,4.1966777 2.35438,9.2362916 0.226456,12.5376596 C 48.750883,2.0172843 47.428284,1.711591 46.273,1.30885 Z\" />",
    "<path id=\"stroke-3\" d=\"m 48.912675,25.437236 c -0.971854,15.434662 -11.121227,12.293489 -23.14783,17.392075 -14.248849,3.330562 -31.6143565,15.710521 -45.253383,5.911035 -7.414644,-7.554927 -20.428212,1.134862 -17.402389,-18.618008 -8.407549,-9.3515 -16.492269,-12.394635 -14.44589,-21.3947098 1.450007,-6.6542624 6.58843,-8.15769866 -0.648885,-17.7023052 -3.69553,-14.085295 9.755107,-30.341847 18.928697,-29.295234 14.011651,-3.414073 26.9089315,-7.172641 40.7256401,-8.976395 15.5810439,-3.566203 32.3391309,9.923948 32.5923619,18.234641 7.697508,8.322364 3.41215,13.901682 10.395603,30.2374764 C 40.43043,10.248548 45.7314,22.03557 48.912675,25.437236 Z\" />",
];

pub struct Stroke {
    stroke_idx: usize,
    x: i32,
    y: i32,
    rotation: i32,
    scale_x: f32,
    scale_y: f32,
    color: Rgba,
}

impl Stroke {

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            stroke_idx: rng.gen_range(0..STROKES.len()) as usize,
            x: 0,
            y: 0,
            rotation: 0,
            scale_x: 1.0,
            scale_y: 1.0,
            color: Rgba::new_black(),
        }
    }

    pub fn get_xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_xy(&mut self, xy: (i32, i32)) {
        self.x = xy.0;
        self.y = xy.1;
    }

    pub fn get_scale(&self) -> (f32, f32) {
        (self.scale_x, self.scale_y)
    }

    pub fn set_scale(&mut self, scale: (f32, f32)) {
        self.scale_x = scale.0;
        self.scale_y = scale.1;
    }

    pub fn set_rotation(&mut self, rotation: i32) {
        self.rotation = rotation;
    }

    pub fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }

    pub fn express(&self) -> String {
        let stroke = format!("<use href=\"#stroke-{}\"/>", self.stroke_idx);
        let transformations = format!("translate({} {}) rotate({}) scale({:.5} {:.5})", self.x, self.y, self.rotation, self.scale_x, self.scale_y);
        let color = self.color.as_hex();
        format!("<g fill=\"{color}\" transform=\"{transformations}\">{stroke}</g>")
    }

    pub fn mutate(&mut self, controller: &Controller) {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=1) {
            0 => {
                let m = controller.get_mutation_movement();
                self.x += rng.gen_range(-m.0..m.1);
                self.y += rng.gen_range(-m.0..m.1);
            },
            1 => {
                self.rotation += rng.gen_range(-90..90);
                self.rotation %= 360;
            },
            _ => panic!("Should be impossible. Check if range of random number properly matches the available options."),
        }
    }

    pub fn approximate_pixels(&self) -> Vec<(i32, i32)> {
        let (width, height) = (
            ((STROKE_DIMENSION.0 * self.scale_x)/2.0).floor() as i32,
            ((STROKE_DIMENSION.1 * self.scale_y)/2.0).floor() as i32,
        );

        let mut pixels = Vec::new();
        for i in -width..width {
            for j in -height..height {
                pixels.push((self.x + i, self.y + j));
            }
        }
        pixels
    }
}

impl Clone for Stroke {

    fn clone(&self) -> Self {
        Self {
            stroke_idx: self.stroke_idx,
            x: self.x,
            y: self.y,
            rotation: self.rotation,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            color: self.color.clone(),
        }
    }
}
