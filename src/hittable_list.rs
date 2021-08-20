use crate::hittable::{Hittable, HitRecord};
use crate::ray::*;
use std::rc::Rc;
use crate::material::{Material, Lambertian, Metal, Dielectric};
use crate::color::Color;
use crate::sphere::Sphere;
use crate::vec3::{Vec3, Point3};
use crate::lib;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList{
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) 
    {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord)-> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add( Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material) ) );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = lib::random_double();
            let center = Point3::new(a as f64 + 0.9*lib::random_double(), 0.2, b as f64 + 0.9*lib::random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add( Box::new(Sphere::new(center, 0.2, sphere_material) ) );
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_vec3(0.5, 1.0);
                    let fuzz = lib::random_double_with_bounds(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add( Box::new(Sphere::new(center, 0.2, sphere_material) ) );
                } else {
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add( Box::new(Sphere::new(center, 0.2, sphere_material) ) );
                }
            }
        }
    }

    let material_1 = Rc::new(Dielectric::new(1.5));
    world.add( Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_1) ) );

    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add( Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_2) ) );

    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add( Box::new(Sphere::new( Point3::new(4.0, 1.0, 0.0), 1.0, material_3) ) );

    world
}