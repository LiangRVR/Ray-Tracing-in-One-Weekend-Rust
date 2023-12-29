use rand::Rng;
use crate::interval::Interval;

//Constants
pub const INFINITY: f64 = std::f64::INFINITY;
pub const NEG_INFINITY: f64 = std::f64::NEG_INFINITY;
pub const PI: f64 = 3.1415926535897932385;
pub const EMPTY: Interval = Interval::empty();
pub const UNIVERSE: Interval = Interval::universe();

//Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * PI) / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
