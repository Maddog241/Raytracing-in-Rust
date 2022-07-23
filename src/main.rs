mod aabb;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod objects;
mod ray;
mod texture;
mod utilities;

use std::fs::File;
use std::io::Write;
use std::env;
use std::process::exit;
use cgmath::*;
use crate::camera::Camera;
use crate::color::*;


pub fn run(arg: i32, path: &str) -> std::io::Result<()>{
    let mut aspect_ratio: f64 = 3.0 / 2.0;
    let mut image_height: usize = 400;
    let mut image_width: usize = (image_height as f64 * aspect_ratio) as usize;
    let mut samples_per_pixel: usize = 20;
    let max_depth: usize = 20;

    //World
    let world;
    let lookfrom;
    let lookat;
    let fov;
    let mut aperture = 0.0;
    let mut background = SKYBLUE;

    match arg {
        1 => {
            world = hittable_list::random_scene();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            fov = 20.0;
            aperture = 0.1;
        }

        2 => {
            world = hittable_list::two_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            fov = 20.0;
            aperture = 0.1;
        }

        3 => {
            background = BLACK;
            world = hittable_list::simple_light();
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            fov = 20.0;
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
            fov = 40.0;
        }
        5 => {
            world = hittable_list::two_perlin_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            fov = 20.0;
        }

        6 => {
            world = hittable_list::earth();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            fov = 20.0;
        }

        7 => {
            aspect_ratio = 1.0;
            image_height = 600;
            image_width = (image_height as f64 * aspect_ratio) as usize;
            samples_per_pixel = 20;
            world = hittable_list::cornell_smoke();
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            fov = 40.0;
        }
        _ => {
            panic!("no such mode");
        }
    }
    //Camera

    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );    
    
    //Render

    let mut file = File::create(path)?;
    let mut output = format!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {} \r", j);
        
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + utilities::random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + utilities::random_double()) / (image_height - 1) as f64;
                let r = cam.cast_ray(u, v);
                pixel_color += color::ray_color(&r, background, &world, max_depth);
            }
            output += &color::write_color(pixel_color, samples_per_pixel);
        }
    }

    file.write(output.as_bytes())?;

    eprintln!("Done");

    Ok(())
}

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: [executable] [image-name]");
        drop(args);
        exit(1);
    }

    let now = std::time::Instant::now();
    let mode = 1;
    let path = String::from("./images/") + &args[1];
    run(mode, &path)?;

    let cost = now.elapsed().as_millis();

    eprintln!("cost: {} secs", cost as f64 / 1000.0);

    Ok(())
}
