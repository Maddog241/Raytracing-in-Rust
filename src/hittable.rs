use crate::ray;
use crate::vec3::{self, Point3, Vec3};
use crate::material::{ Material};
use std::rc::Rc;
//use crate::sphere::Sphere;

//Trait Hittable
pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec:&mut HitRecord) -> bool;
}

//HitRecord
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(mat_ptr: Rc<dyn Material>) -> Self {
        HitRecord{
            p: Point3{x:0.0, y:0.0, z:0.0},
            normal: Vec3{x:0.0, y:0.0, z:0.0},
            t: 0.0,
            front_face: true,
            mat_ptr,
        }
    }


    pub fn set_face_normal(&mut self, r:&ray::Ray, outward_normal: vec3::Vec3) {
        if vec3::dot(r.direction(), outward_normal) > 0.0 {
            self.front_face = false;
            self.normal = outward_normal*-1.0;
        } else {
            self.front_face = true;
            self.normal = outward_normal;
        }
    }
}
