use cgmath::*;

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
    pub time: f64, //added for motion_blur
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time, // the time the single ray exists.
        }
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction * t
    }
}
