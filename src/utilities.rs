use rand::Rng;
use std::f64::{self, consts};
use cgmath::*;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * consts::PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0, f64::MAX) / f64::MAX
}

pub fn random_double_with_bounds(lower: f64, upper: f64) -> f64 {
    lower + (upper - lower) * random_double()
}

pub fn unit_vector(v: Vector3<f64>) -> Vector3<f64> {
    v / v.magnitude()
}

pub fn random_vec3(lower: f64, upper: f64) -> Vector3<f64> {
    Vector3::new(
        random_double_with_bounds(lower, upper),
        random_double_with_bounds(lower, upper),
        random_double_with_bounds(lower, upper),
    )
}

pub fn random() -> Vector3<f64> {
    Vector3::new(random_double(), random_double(), random_double())
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = random_vec3(-1.0, 1.0);
        if p.magnitude2() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vector3<f64> {
    unit_vector(random_in_unit_sphere())
}

pub fn clamp(x: f64, lower: f64, upper: f64) -> f64 {
    if x < lower {
        return lower;
    } else if x > upper {
        return upper;
    }
    x
}

pub fn random_int_with_bounds(lower: i32, upper: i32) -> i32 {
    let range = (upper - lower) as usize;
    let rand_number = rand::random::<usize>() % range;
    rand_number as i32 + lower
}

pub fn near_zero(v: Vector3<f64>) -> bool {
    if v.magnitude() < 0.01 { true } else { false }
}


