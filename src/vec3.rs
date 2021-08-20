use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use crate::lib;
//use crate::hittable::{HitRecord};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

// default initialization
// get x, y, z coordinate
// operator + , -, *, /, += , -=, *=, /=
// dot pro, cross pro
// get length, get length_squared


impl Vec3 {
    pub fn new(x: f64, y: f64, z:f64) -> Self {
        Vec3 {
            x, y, z,
        }
    }

    pub fn length_squared(&self) -> f64 {
        let tot = 
            self.x * self.x + 
            self.y * self.y + 
            self.z * self.z ;
        tot
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn random_vec3(lower: f64, upper: f64) -> Vec3 {
        Vec3 {
            x: lib::random_double_with_bounds(lower, upper),
            y: lib::random_double_with_bounds(lower, upper),
            z: lib::random_double_with_bounds(lower, upper),
        }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: lib::random_double(),
            y: lib::random_double(),
            z: lib::random_double(),
        }
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_vec3(-1.0, 1.0);
            if p.length_squared() >= 1.0 { continue; }
            return p
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        unit_vector(Self::random_in_unit_sphere())
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs()<s && self.y.abs()<s && self.z.abs()<s
    }

}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {

    fn add_assign(&mut self, rhs: Self){
        self.x += rhs.x ;
        self.y += rhs.y ;
        self.z += rhs.z ;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    
    fn sub_assign(&mut self, rhs:Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
/*
impl Mul for Vec3 {
    type Output = Self ;

    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
*/
impl Mul<f64> for Vec3{
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}
/*
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self){
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
*/

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

pub fn cross(lhs:Vec3, rhs:Vec3) -> Vec3 {
    Vec3{
        x: lhs.y * rhs.z - lhs.z * rhs.y,
        y: lhs.z * rhs.x - lhs.x * rhs.z,
        z: lhs.x * rhs.y - lhs.y * rhs.x,
    }
}

pub fn dot(lhs:Vec3, rhs:Vec3) -> f64 {
    lhs.x * rhs.x + 
    lhs.y * rhs.y + 
    lhs.z * rhs.z
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub type Point3 = Vec3;

