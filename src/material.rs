use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::utilities::*;
use std::rc::Rc;
use cgmath::*;

//Definition of Material
pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
    
    fn emitted(&self, _u: f64, _v: f64, _p: Point3<f64>) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

//Metal
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(r_in.direction), rec);
        scattered.origin = rec.p;
        scattered.direction = reflected + random_in_unit_sphere() * self.fuzz;
        scattered.time = r_in.time;

        *attenuation = self.albedo.clone();
        scattered.direction.dot(rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

//Lambertian
pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Lambertian {
            albedo: Rc::new(SolidColor::new(a)),
        }
    }

    pub fn new_texture(a: Rc<dyn Texture>) -> Self {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        scattered.origin = rec.p;
        scattered.direction = scatter_direction;
        scattered.time = r_in.time;

        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);

        true
    }
}

//Dielectric
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = unit_vector(r_in.direction);
        let cos_theta = min(-r_in.direction.dot(rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = sin_theta * refraction_ratio > 1.0;

        let direction = if cannot_refract {
            reflect(unit_direction, rec)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };
        scattered.origin = rec.p;
        scattered.direction = direction;
        scattered.time = r_in.time;
        true
    }
}

//struct DiffuseLight
pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Rc<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: Point3<f64>) -> Color {
        self.emit.value(u, v, p)
    }
}

//Other funtions
pub fn reflect(v: Vector3<f64>, rec: &HitRecord) -> Vector3<f64> {
    v - rec.normal * v.dot(rec.normal) * 2.0
}

pub fn min(v1: f64, v2: f64) -> f64 {
    if v1 < v2 {
        v1
    } else {
        v2
    }
}

pub fn abs(v: f64) -> f64 {
    if v < 0.0 {
        -v
    } else {
        v
    }
}


// !!!!!????????????
pub fn refract(unit_direction: Vector3<f64>, normal: Vector3<f64>, ratio: f64) -> Vector3<f64> {
    let cos_theta = min(-unit_direction.dot(normal), 1.0);
    let r_out_prep = (unit_direction + normal * cos_theta) * ratio;
    let r_out_parallel = normal * (-1.0) * (abs(1.0 - r_out_prep.magnitude2())).sqrt();
    r_out_prep + r_out_parallel
}


pub struct Isotropic {
    albedo: Box<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Isotropic { albedo }
    }
}
impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *scattered = Ray::new(rec.p, random_in_unit_sphere(), r_in.time);
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }   
}
