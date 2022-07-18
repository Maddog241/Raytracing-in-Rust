use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::objects::{Cubic, MovingSphere, RotateY, Sphere, XyRect, XzRect, YzRect, Translate, ConstantMedium};
use crate::ray::*;
use crate::texture::{CheckerTexture, NoiseTexture, SolidColor, ImageTexture};
use crate::utilities;
use std::rc::Rc;
use cgmath::*;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    /*
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord)-> Option<HitRecord> {
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
    */

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = record.t;
                rec = Some(record);
            }
        }

        rec
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.objects.len() == 0 {
            return false;
        }

        let mut temp_box: AABB = AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        let mut first_box = true;

        for object in self.objects.iter() {
            if object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box.clone()
            } else {
                AABB::surrounding_box(&output_box, &temp_box)
            };
            first_box = false;
        }

        true
    }
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = CheckerTexture::new(Box::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))), Box::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_texture(Rc::new(checker))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utilities::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * utilities::random_double(),
                0.2,
                b as f64 + 0.9 * utilities::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = utilities::random().mul_element_wise(utilities::random());
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    let center2 = center
                        + Vector3::new(0.0, utilities::random_double_with_bounds(0.0, 0.5), 0.0);
                    world.add(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = utilities::random_vec3(0.5, 1.0);
                    let fuzz = utilities::random_double_with_bounds(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    world
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker1 = CheckerTexture::new(Box::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))), Box::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))));
    let checker2 = CheckerTexture::new(Box::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))), Box::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new_texture(Rc::new(checker1))),
    )));

    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new_texture(Rc::new(checker2))),
    )));

    objects
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_texture(Rc::new(NoiseTexture::new(4.0)))),
    )));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new_texture(Rc::new(NoiseTexture::new(4.0)))),
    )));

    let light = Color::new(15.0, 15.0, 15.0);
    let difflight = Rc::new(DiffuseLight::new(Rc::new(SolidColor::new(light))));
    objects.add(Box::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    objects
}

pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Color::new(0.65, 0.05, 0.05);
    let white = Color::new(0.73, 0.73, 0.73);
    let green = Color::new(0.12, 0.45, 0.15);
    let light = Color::new(15.0, 15.0, 15.0);

    objects.add(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Rc::new(Lambertian::new(green)))));
    objects.add(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0,   Rc::new(Lambertian::new(red)))));
    objects.add(Box::new(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, Rc::new(DiffuseLight::new(Rc::new(SolidColor::new(light)))))));
    objects.add(Box::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Rc::new(Lambertian::new(white)))));
    objects.add(Box::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Rc::new(Lambertian::new(white)))));
    objects.add(Box::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Rc::new(Lambertian::new(white)))));

    let cub1 = Cubic::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), Rc::new(Lambertian::new(white)));
    let cub2 = Cubic::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), Rc::new(Lambertian::new(white)));

    let rotated_cub1 = RotateY::new(Box::new(cub1), 18.0);
    let rotated_cub2 = RotateY::new(Box::new(cub2), -15.0);

    let translated_cub1 = Translate::new(Box::new(rotated_cub1), Vector3::new(265.0, 0.0, 295.0));
    let translated_cub2 = Translate::new(Box::new(rotated_cub2), Vector3::new(130.0, 0.0, 65.0));

    objects.add(Box::new(translated_cub1));
    objects.add(Box::new(translated_cub2));
    objects
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_texture(Rc::new(NoiseTexture::new(4.0)))),
    )));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new_texture(Rc::new(NoiseTexture::new(4.0)))),
    )));

    objects
}

pub fn earth() -> HittableList {
    let earth_texture = ImageTexture::new("./images/earthmap.jpg");
    let earth_surface = Lambertian::new_texture(Rc::new(earth_texture));
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, Rc::new(earth_surface));

    HittableList{
        objects: vec![Box::new(globe)],
    }

}

pub fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red = Color::new(0.65, 0.05, 0.05);
    let white = Color::new(0.73, 0.73, 0.73);
    let green = Color::new(0.12, 0.45, 0.15);
    let light = Color::new(15.0, 15.0, 15.0);

    objects.add(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Rc::new(Lambertian::new(green)))));
    objects.add(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0,   Rc::new(Lambertian::new(red)))));
    objects.add(Box::new(XzRect::new(113.0, 443.0, 127.0, 432.0, 554.0, Rc::new(DiffuseLight::new(Rc::new(SolidColor::new(light)))))));
    objects.add(Box::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Rc::new(Lambertian::new(white)))));
    objects.add(Box::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Rc::new(Lambertian::new(white)))));
    objects.add(Box::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Rc::new(Lambertian::new(white)))));

    let cub1 = Cubic::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), Rc::new(Lambertian::new(white)));
    let cub2 = Cubic::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), Rc::new(Lambertian::new(white)));

    let rotated_cub1 = RotateY::new(Box::new(cub1), 18.0);
    let rotated_cub2 = RotateY::new(Box::new(cub2), -15.0);

    let translated_cub1 = Translate::new(Box::new(rotated_cub1), Vector3::new(265.0, 0.0, 295.0));
    let translated_cub2 = Translate::new(Box::new(rotated_cub2), Vector3::new(130.0, 0.0, 65.0));

    objects.add(Box::new(ConstantMedium::new(Box::new(translated_cub1), Box::new(SolidColor::new(Color::new(0.0, 0.0, 0.0))),  0.01)));
    objects.add(Box::new(ConstantMedium::new(Box::new(translated_cub2), Box::new(SolidColor::new(Color::new(1.0, 1.0, 1.0))),  0.01)));

    objects
}