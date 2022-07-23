use crate::ray::Ray;
use crate::utilities;
use cgmath::*;

pub struct Camera {
    origin: Point3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lower_left_corner: Point3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3<f64>,
        look_at: Point3<f64>,
        vup: Vector3<f64>,
        fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Camera {
        //look_from: the position fo the camera
        //look_at: the view direction
        //vup: the up vector
        //time0 - time1: the time interval when the camera send out rays.
        let theta = utilities::degrees_to_radians(fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = utilities::unit_vector(look_from - look_at);
        let u = utilities::unit_vector(vup.cross(w));
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: look_from - horizontal / 2.0 - vertical / 2.0 - w * focus_dist,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn cast_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_dist() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
            time: utilities::random_double_with_bounds(self.time0, self.time1),
        }
    }
}

pub fn random_in_unit_dist() -> Vector3<f64> {
    loop {
        let p = Vector3::new(
            utilities::random_double_with_bounds(-1.0, 1.0),
            utilities::random_double_with_bounds(-1.0, 1.0),
            0.0,
        );
        if p.magnitude2() >= 1.0 {
            continue;
        }
        return p;
    }
}

