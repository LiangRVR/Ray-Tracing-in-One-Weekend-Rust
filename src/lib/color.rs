use crate::interval::Interval;

use super::vec3::Vec3 as vec3;

pub type Color = vec3;

impl Color {
    pub fn write_color(&self, samples_per_pixel: i32) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // Divide the color by the number of samples
        let scale = 1.0 / (samples_per_pixel as f64);
        r *= scale;
        g *= scale;
        b *= scale;

        let intensity = Interval::new(0.000, 0.999);

        let ir = (256.000 * intensity.clamp(r)) as i32;
        let ig = (256.000 * intensity.clamp(g)) as i32;
        let ib = (256.000 * intensity.clamp(b)) as i32;

        println!("{} {} {}", ir, ig, ib);
    }
}
