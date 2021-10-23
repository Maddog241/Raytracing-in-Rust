use crate::camera::Camera;
use crate::color::{self, Color};
use crate::hittable_list::{self};
use crate::utilities;
use crate::vec3::{Point3, Vec3};

pub fn run(arg: i32) {
    let mut aspect_ratio: f64 = 3.0 / 2.0;
    let mut image_height: usize = 400;
    let mut image_width: usize = (image_height as f64 * aspect_ratio) as usize;
    let mut samples_per_pixel: usize = 100;
    let max_depth: usize = 20;

    //World
    let world;
    let lookfrom;
    let lookat;
    let vfov;
    let mut aperture = 0.0;
    let mut background = Color::skyblue();

    match arg {
        1 => {
            world = hittable_list::random_scene();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }

        2 => {
            world = hittable_list::two_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }

        3 => {
            background = Color::black();
            world = hittable_list::simple_light();
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }

        4 => {
            aspect_ratio = 1.0;
            image_height = 600;
            image_width = (image_height as f64 * aspect_ratio) as usize;
            samples_per_pixel = 500;
            world = hittable_list::cornell_box();
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        5 => {
            world = hittable_list::two_perlin_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }

        6 => {
            world = hittable_list::earth();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }

        7 => {
            aspect_ratio = 1.0;
            image_height = 600;
            image_width = (image_height as f64 * aspect_ratio) as usize;
            samples_per_pixel = 500;
            world = hittable_list::cornell_smoke();
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            panic!("no such mode");
        }
    }
    //Camera

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    /*
    println!("origin: {:#?}", origin);
    println!("horizontal: {:#?}", horizontal);
    println!("vertical: {:#?}", vertical);
    println!("{:#?}", lower_left_corner);
    */

    //Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {} \r", j);
        
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + utilities::random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + utilities::random_double()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += color::ray_color(&r, background, &world, max_depth);
            }
            color::write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done");
}
