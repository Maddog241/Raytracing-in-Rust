use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vec3::{self, Vec3};

//Definition of Material
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool ;
}

//DefaultMaterial
pub struct DefaultMaterial {}

impl DefaultMaterial {
    pub fn new() -> Self {
        DefaultMaterial{ }
    }
} 

impl Material for DefaultMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool { false }
}



//Metal
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(vec3::unit_vector(r_in.direction()), rec);
        
        scattered.orig = rec.p;
        scattered.dir = reflected + Vec3::random_in_unit_sphere()*self.fuzz;

        *attenuation = self.albedo.clone();
        vec3::dot( scattered.direction(), rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz,
        }
    }
}

//Lambertian
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal;
        }

        scattered.orig = rec.p;
        scattered.dir = scatter_direction;

        *attenuation = self.albedo.clone();

        true
    }

}

//Dielectric
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric{
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color{x:1.0, y:1.0, z:1.0};
        let refraction_ratio = if rec.front_face { 1.0/self.index_of_refraction } else {self.index_of_refraction};

        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = min(vec3::dot(r_in.direction()*(-1.0), rec.normal), 1.0);
        let sin_theta = (1.0-cos_theta*cos_theta).sqrt();

        let cannot_refract = sin_theta * refraction_ratio > 1.0 ;

        let direction = if cannot_refract { reflect(unit_direction, rec)} else { refract(unit_direction, rec.normal, refraction_ratio)};
    
        scattered.orig = rec.p;
        scattered.dir = direction;
        
        true
    }
}

//Other funtions
pub fn reflect(v: Vec3, rec: &HitRecord) -> Vec3 {
    v - rec.normal * vec3::dot(v, rec.normal)*2.0
}

pub fn min(v1:f64, v2:f64) -> f64 {
    if v1 < v2 { v1 } else { v2 }
}

pub fn abs(v: f64) -> f64 {
    if v < 0.0 { -v } else { v }
}

// !!!!!折射函数
pub fn refract(unit_direction: Vec3, normal: Vec3, ratio: f64) -> Vec3 {
    let cos_theta = min(vec3::dot(unit_direction*(-1.0), normal), 1.0);
    let r_out_prep = (unit_direction + normal*cos_theta) * ratio;
    let r_out_parallel = normal * (-1.0) * (abs(1.0-r_out_prep.length_squared())).sqrt();
    r_out_prep + r_out_parallel
}