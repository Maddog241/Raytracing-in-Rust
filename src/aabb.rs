use crate::ray::Ray;
use cgmath::Point3;

#[derive(Clone)]
pub struct AABB {
    minimum: Point3<f64>,
    maximum: Point3<f64>,
}

impl AABB {
    pub fn new(a: Point3<f64>, b: Point3<f64>) -> Self {
        AABB {
            minimum: a,
            maximum: b,
        }
    }

    pub fn min(&self) -> Point3<f64> {
        self.minimum
    }

    pub fn max(&self) -> Point3<f64> {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: &mut f64, t_max: &mut f64) -> bool {
        //在三个方向均有overlap
        for i in 0..3 {
            let inv_d = 1.0 / r.direction[i];  //int rust, the 'zero-divison error' won't occur because the answer would be 'inf', '-inf' or 'NaN'.
            let mut t0 = (self.min()[i] - r.origin[i]) * inv_d;
            let mut t1 = (self.max()[i] - r.origin[i]) * inv_d;
            if inv_d < 0.0 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }

            *t_min = if t0 > *t_min { t0 } else { *t_min };
            *t_max = if t1 < *t_max { t1 } else { *t_max };
            if t_min >= t_max {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        let small = Point3::new(
            box0.min().x.min(box1.min().x),
            box0.min().y.min(box1.min().y),
            box0.min().z.min(box1.min().z),
        );

        let big = Point3::new(
            box0.max().x.max(box1.max().x),
            box0.max().y.max(box1.max().y),
            box0.max().z.max(box1.max().z),
        );

        AABB::new(small, big)
    }
}
