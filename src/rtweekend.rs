pub const infinity: f64 = f64::INFINITY;
pub const pi: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * pi / 180.0;
}

pub fn random_double() -> f64 {
    return rand::random();
}

pub fn random_within_range(min: f64, max: f64) -> f64 {
    return min + (max - min) * random_double();
}