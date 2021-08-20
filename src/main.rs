mod vec3;
mod color;
mod ray;
mod hittable_list;
mod sphere;
mod hittable;
mod lib;
mod camera;
mod material;

use color::Color;
use hittable::{HitRecord};
use hittable_list::HittableList;
use ray::Ray;
use vec3::Vec3;
use hittable::Hittable;
use camera::Camera;
use material::DefaultMaterial;
use std::rc::Rc;

fn ray_color(r:&Ray, world: &HittableList, depth : usize) -> Color {
    if depth == 0 { return Color{x:1.0, y:1.0, z:1.0}; }

    let mut rec: HitRecord = HitRecord::new( Rc::new(DefaultMaterial::new()));
    if world.hit(r, 0.001, std::f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(rec.p, rec.normal);
        let mut attenuation = Color{x:0.0, y:0.0, z:0.0};
        if rec.mat_ptr.scatter(&r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1);
        }
        return Color{x:0.0, y:0.0, z:0.0};
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5*(unit_direction.y + 1.0);
    Color{x:1.0, y:1.0, z:1.0}*(1.0-t) + Color{x:0.5, y:0.7, z:1.0}*t
}

fn main() {

    //Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_HEIGHT: usize = 800;
    const IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 20;

    //World
    let world = hittable_list::random_scene();

    //Camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(look_from, look_at, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);
    /*
    println!("origin: {:#?}", origin);
    println!("horizontal: {:#?}", horizontal);
    println!("vertical: {:#?}", vertical);
    println!("{:#?}", lower_left_corner);
    */

    //Render 

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);


        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color{x:0.0, y:0.0, z:0.0};

            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + lib::random_double()) / (IMAGE_WIDTH -1) as f64 ;
                let v = (j as f64 + lib::random_double()) / (IMAGE_HEIGHT-1) as f64 ;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(pixel_color, SAMPLES_PER_PIXEL);
        }

    }
    eprintln!("Done");

}

/*
fn main(){
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut i = 0;
    while i < IMAGE_WIDTH {
        let mut j = IMAGE_HEIGHT;
        while j >= 0  {
            let r = (i as f64) / (IMAGE_WIDTH as f64 - 1.0);
            let g = (j as f64) / (IMAGE_HEIGHT as f64 - 1.0);
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);

            j -= 1;
        } 
        i += 1;
    }
}
*/