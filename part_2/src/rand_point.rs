use rand;
use rand::prelude::ThreadRng;
use rand::Rng;

struct Solution {
    radius: f64,
    radius_2: f64,
    x_center: f64,
    y_center: f64,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            radius_2: radius * radius,
            x_center,
            y_center,
            rng: rand::thread_rng(),
        }
    }

    fn rand_point(&mut self) -> Vec<f64> {
        let mut x_delta = self.rng.gen_range(-self.radius..=self.radius);
        let mut y_delta = self.rng.gen_range(-self.radius..=self.radius);
        while x_delta * x_delta + y_delta * y_delta > self.radius_2 {
            x_delta = self.rng.gen_range(-self.radius..=self.radius);
            y_delta = self.rng.gen_range(-self.radius..=self.radius);
        }
        return vec![self.x_center + x_delta, self.y_center + y_delta];
    }
}
