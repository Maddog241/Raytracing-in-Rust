use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Material, Isotropic};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::utilities;
use std::rc::Rc;
use cgmath::*;

const PI: f64 = std::f64::consts::PI;
//use crate::ray::Ray;
//use crate::color::Color;

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }

    fn get_sphere_uv(p: Vector3<f64>) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rec: HitRecord;

        let oc = r.origin - self.center;
        let a = r.direction.magnitude2();
        let half_b = oc.dot(r.direction);
        let c = oc.magnitude2() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal :Vector3<f64> = (r.at(root) - self.center) / self.radius;
        let (u, v) = Self::get_sphere_uv(outward_normal);

        rec = HitRecord::new(
            root,
            r.at(root),
            r,
            outward_normal,
            u,
            v,
            Rc::clone(&self.mat_ptr),
        );
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vector3::new(self.radius, self.radius, self.radius),
            self.center + Vector3::new(self.radius, self.radius, self.radius),
        );

        true
    }
}

//moving Sphere
pub struct MovingSphere {
    pub center0: Point3<f64>,
    pub center1: Point3<f64>,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3<f64>,
        center1: Point3<f64>,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: Rc<dyn Material>,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr,
        }
    }

    pub fn center(&self, time: f64) -> Point3<f64> {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Material for MovingSphere {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        true
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.magnitude2();
        let half_b = oc.dot(r.direction);
        let c = oc.magnitude2() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal = (r.at(root) - self.center(r.time)) / self.radius;

        let rec = HitRecord::new(
            root,
            r.at(root),
            r,
            outward_normal,
            0.0,
            0.0,
            Rc::clone(&self.mat_ptr),
        );
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB::new(
            self.center(time0) - Vector3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vector3::new(self.radius, self.radius, self.radius),
        );

        let box1 = AABB::new(
            self.center(time1) - Vector3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vector3::new(self.radius, self.radius, self.radius),
        );

        *output_box = AABB::surrounding_box(&box0, &box1);
        true
    }
}

//struct XyRect
pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mp: Rc<dyn Material>,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Rc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / (r.direction.z);
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vector3::new(0.0, 0.0, 1.0);
        
        let rec = HitRecord::new(t, r.at(t), r, outward_normal, u, v, Rc::clone(&self.mp));

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}

//struct YzRect
pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Rc<dyn Material>,
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Rc<dyn Material>) -> Self {
        YzRect {
            y0,
            y1,
            z0,
            z1,
            k,
            mp,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.x) / (r.direction.x);
        if t < t_min || t > t_max {
            return None;
        }

        let y = r.origin.y + t * r.direction.y;
        let z = r.origin.z + t * r.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vector3::new(1.0, 0.0, 0.0);

        let rec = HitRecord::new(t, r.at(t), r, outward_normal, u, v, Rc::clone(&self.mp));

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }
}

//struct XzRect
pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Rc<dyn Material>,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Rc<dyn Material>) -> Self {
        XzRect {
            x0,
            x1,
            z0,
            z1,
            k,
            mp,
        }
    }
}
impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / (r.direction.y);
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vector3::new(0.0, 1.0, 0.0);

        let rec = HitRecord::new(t, r.at(t), r, outward_normal, u, v, Rc::clone(&self.mp));
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z0),
        );
        true
    }
}

//struct Cubic
pub struct Cubic {
    sides: HittableList,
    p1: Point3<f64>,
    p2: Point3<f64>,
}

impl Cubic {
    pub fn new(p1: Point3<f64>, p2: Point3<f64>, mat_ptr: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        //front and back
        sides.add(Box::new(XyRect::new(
            p1.x,
            p2.x,
            p1.y,
            p2.y,
            p1.z,
            Rc::clone(&mat_ptr),
        )));
        sides.add(Box::new(XyRect::new(
            p1.x,
            p2.x,
            p1.y,
            p2.y,
            p2.z,
            Rc::clone(&mat_ptr),
        )));
        //up and down
        sides.add(Box::new(XzRect::new(
            p1.x,
            p2.x,
            p1.z,
            p2.z,
            p1.y,
            Rc::clone(&mat_ptr),
        )));
        sides.add(Box::new(XzRect::new(
            p1.x,
            p2.x,
            p1.z,
            p2.z,
            p2.y,
            Rc::clone(&mat_ptr),
        )));
        //left and right
        sides.add(Box::new(YzRect::new(
            p1.y,
            p2.y,
            p1.z,
            p2.z,
            p1.x,
            Rc::clone(&mat_ptr),
        )));
        sides.add(Box::new(YzRect::new(
            p1.y,
            p2.y,
            p1.z,
            p2.z,
            p2.x,
            Rc::clone(&mat_ptr),
        )));
        Cubic {
            sides,
            p1,
            p2,
        }
    }
}

impl Hittable for Cubic {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.p1, self.p2);
        true
    }
}

//struct rotate_y
pub struct RotateY {
    ptr: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    has_box: bool,
    bbox: AABB,
}

impl RotateY {
    pub fn new(ptr: Box<dyn Hittable>, angle: f64) -> Self {
        let angle = utilities::degrees_to_radians(angle);
        let sin_theta = angle.sin();
        let cos_theta = angle.cos();

        let mut bbox = AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        let has_box = ptr.bounding_box(0.0, 1.0, &mut bbox);

        let mut minimum = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut maximum = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.max().x + (1.0 - i as f64) * bbox.min().x;
                    let y = (j as f64) * bbox.max().y + (1.0 - j as f64) * bbox.min().y;
                    let z = (k as f64) * bbox.max().z + (1.0 - k as f64) * bbox.min().z;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Point3::new(newx, y, newz);

                    for c in 0..3 {
                        minimum[c] = tester[c].min(minimum[c]);
                        maximum[c] = tester[c].min(maximum[c]);
                    }
                }
            }
        }

        bbox = AABB::new(minimum, maximum);

        RotateY {
            ptr,
            sin_theta,
            cos_theta,
            bbox,
            has_box,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        let mut origin = r.origin;
        let mut direction = r.direction;

        origin[0] = origin[0] * self.cos_theta - origin[2] * self.sin_theta;
        origin[2] = origin[0] * self.sin_theta + origin[2] * self.cos_theta;

        direction[0] = self.cos_theta * direction[0] - self.sin_theta * direction[2];
        direction[2] = self.sin_theta * direction[0] + self.cos_theta * direction[2];

        let rotated_r = Ray::new(origin, direction, r.time);

        match self.ptr.hit(&rotated_r, t_min, t_max) {
            None => None,
            Some(mut rec) => {
                let mut p = rec.p;
                let mut normal = rec.normal;

                p[0] =  self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
                p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

                normal[0] =  self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
                normal[2] = -self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];

                rec.p = p;
                rec.set_face_normal(&rotated_r, normal);

                Some(rec)
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox.clone();
        true
    }
}

pub struct Translate{
    obj: Box<dyn Hittable>,
    offset: Vector3<f64>,
}

impl Translate {
    pub fn new(obj: Box<dyn Hittable>, offset: Vector3<f64>) -> Self {
        Translate {
            obj,
            offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self ,r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(r.origin-self.offset, r.direction, r.time);
        match self.obj.hit(&moved_ray, t_min, t_max) {
            Some(mut rec) => {
                rec.p = rec.p + self.offset;
                Some(rec)
            },
            None => None,
        }

    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if !self.obj.bounding_box(time0, time1, output_box) {
            return false;
        } 

        *output_box  = AABB::new(output_box.min()+self.offset, output_box.max()+self.offset);
        true
    }
}

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    phase_function: Rc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>, text: Box<dyn Texture> , distance: f64) -> Self {
        ConstantMedium {
            boundary,
            phase_function: Rc::new(Isotropic::new(text)),
            neg_inv_density: -1.0 / distance, 
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (t1, t2);

        match self.boundary.hit(r, t_min, t_max) {
            None => return None,
            Some(rec) => {
                t1 = rec.t;
                match self.boundary.hit(r, t1+0.0001, t_max) {
                    None => return None,
                    Some(rec2) => { t2 = rec2.t; }
                }
            }
        }

        let ray_length = r.direction.magnitude();
        let distance_inside_boundary = (t2 - t1) * ray_length;
        let hit_distance = self.neg_inv_density * utilities::random_double().log10();

        if hit_distance > distance_inside_boundary {
            return None
        }

        let t = t1 + hit_distance / ray_length;
        let p = r.at(t);

        Some(
            HitRecord {
                t,
                p,
                mat_ptr: Rc::clone(&self.phase_function),
                normal: Vector3::new(1.0, 0.0, 0.0),
                front_face: true,
                u: 0.0, 
                v: 0.0,
            }
        )
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}