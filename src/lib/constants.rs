use crate::interval::Interval;

//Constants
pub const INFINITY:f64 = std::f64::INFINITY;
pub const NEG_INFINITY:f64 = std::f64::NEG_INFINITY;
pub const PI:f64 = 3.1415926535897932385;
pub const EMPTY: Interval = Interval::empty();
pub const UNIVERSE: Interval = Interval::universe();

//Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
