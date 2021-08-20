use crate::vec3::Vec3;
use crate::lib;

//display
pub type Color = Vec3;

pub fn write_color(pixel_color: Color, sample_per_pixel: usize) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / (sample_per_pixel as f64);
    r = (scale*r).sqrt();
    g = (scale*g).sqrt();
    b = (scale*b).sqrt();

    println!("{} {} {}"
            ,(256.0 * lib::clamp(r, 0.0, 0.999)) as u32
            ,(256.0 * lib::clamp(g, 0.0, 0.999)) as u32
            ,(256.0 * lib::clamp(b, 0.0, 0.999)) as u32
        )
}

