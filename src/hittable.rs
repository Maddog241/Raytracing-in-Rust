use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
use std::rc::Rc;
//use crate::sphere::Sphere;

//Trait Hittable
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}

//HitRecord
pub struct HitRecord {
    pub p: Point3, //hit point
    pub normal: Vec3,
    pub t: f64, //t is the value of ray.t
    pub u: f64,
    pub v: f64, //u and v are the surface coordinates
    pub front_face: bool,
    pub mat_ptr: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        t: f64,
        p: Point3,
        r: &Ray,
        outward_normal: vec3::Vec3,
        u: f64,
        v: f64,
        mat_ptr: Rc<dyn Material>,
    ) -> Self {

        let mut rec = HitRecord {
            p: r.at(t),
            normal: outward_normal,
            t,
            front_face: true,
            mat_ptr,
            u,
            v,
        };

        rec.set_face_normal(r, outward_normal);
        rec
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: vec3::Vec3) {
        if vec3::dot(r.direction, outward_normal) > 0.0 {
            self.front_face = false;
            self.normal = outward_normal * -1.0;
        } else {
            self.front_face = true;
            self.normal = outward_normal;
        }
    }
}
