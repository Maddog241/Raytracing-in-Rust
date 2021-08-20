use rand::Rng;
use std::f64::{consts, self};


pub fn degrees_to_radians(degrees: f64) -> f64{
    degrees * consts::PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0, f64::MAX) / f64::MAX 
}

pub fn random_double_with_bounds(lower: f64, upper: f64) -> f64 {
    lower + (upper-lower)*random_double()
}

pub fn clamp(x: f64, lower:f64, upper: f64) -> f64 {
    if x < lower { return lower; }
    else if x > upper { return upper; }
    x
}

/*
#[cfg(test)]
mod test {
    #[test]
    
}
*/