use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::utilities;
use cgmath::*;

//display

pub type Color = Vector3<f64>;

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const SKYBLUE: Color = Color::new(0.7, 0.8, 1.0);


pub fn write_color(pixel_color: Color, sample_per_pixel: usize) -> String{
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / (sample_per_pixel as f64);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    format!(
        "{} {} {}\n",
        (256.0 * utilities::clamp(r, 0.0, 0.999)) as u32,
        (256.0 * utilities::clamp(g, 0.0, 0.999)) as u32,
        (256.0 * utilities::clamp(b, 0.0, 0.999)) as u32
    )
}

pub fn ray_color(r: &Ray, background: Color, world: &HittableList, depth: usize) -> Color {
    let rec: HitRecord;
    if depth == 0 {
        return BLACK;
    }

    match world.hit(r, 0.001, std::f64::INFINITY) {
        Some(record) => {
            rec = record;
        }
        None => {
            return background;
        }
    }

    let mut scattered = Ray::new(rec.p, rec.normal, r.time);
    let mut attenuation = WHITE;
    let emitted = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);

    if !rec
        .mat_ptr
        .scatter(r, &rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }

    emitted + ray_color(&scattered, background, world, depth - 1).mul_element_wise(attenuation)
}
