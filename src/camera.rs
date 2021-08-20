use crate::vec3::{self, Point3, Vec3};
use crate::ray::Ray;
use crate::lib;

pub struct Camera {
    origin: Point3,
    horizontal : Vec3,
    vertical : Vec3,
    lower_left_corner : Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3, look_at:Point3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64
    ) -> Camera 
    {
        
            //look_from: the position fo the camera
        //look_at: the view direction 
        //vup: the up vector
        let theta = lib::degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = vec3::unit_vector(look_from-look_at);
        let u = vec3::unit_vector(vec3::cross(vup, w));
        let v = vec3::cross(w, u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: look_from - horizontal/2.0 - vertical/2.0 - w*focus_dist,
            u,
            v,
            w,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_dist() * self.lens_radius ;
        let offset =  self.u * rd.x + self.v * rd.y ;

        Ray {
            orig : self.origin + offset,
            dir : self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset,
        }
    }
}

pub fn random_in_unit_dist() -> Vec3 {
    loop {
        let p = Vec3::new(lib::random_double_with_bounds(-1.0, 1.0), lib::random_double_with_bounds(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
