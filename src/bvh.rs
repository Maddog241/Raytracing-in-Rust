use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::utilities;
use crate::vec3::Point3;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BVHNode {
    pub bound: AABB,
    pub left: Option<Rc<dyn Hittable>>,
    pub right: Option<Rc<dyn Hittable>>,
}

impl BVHNode {
    pub fn new(
        src_objects: &Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let left: Option<Rc<dyn Hittable>>;
        let right: Option<Rc<dyn Hittable>>;
        let axis = utilities::random_int_with_bounds(0, 3);
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = end - start;

        match object_span {
            1 => {
                left = Some(Rc::clone(&src_objects[start]));
                right = Some(Rc::clone(&src_objects[start]));
            }
            2 => {
                if comparator(&src_objects[start], &src_objects[start + 1]) == Ordering::Less {
                    left = Some(Rc::clone(&src_objects[start]));
                    right = Some(Rc::clone(&src_objects[start + 1]));
                } else {
                    left = Some(Rc::clone(&src_objects[start + 1]));
                    right = Some(Rc::clone(&src_objects[start]));
                }
            }
            _ => {
                let mut part = Vec::new();
                for i in start..end {
                    part.push(Rc::clone(&src_objects[i]));
                }
                part.sort_by(comparator);

                let mid = start + object_span / 2;
                left = Some(Rc::new(BVHNode::new(&part, 0, mid - start, time0, time1)));
                right = Some(Rc::new(BVHNode::new(
                    &part,
                    mid - start,
                    end - start,
                    time0,
                    time1,
                )));
            }
        }

        let mut box_left = AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        let mut box_right = AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));

        if !(left.clone().unwrap()).bounding_box(time0, time1, &mut box_left)
            || !(right.clone().unwrap()).bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in BVHNode constructor");
        }

        let bound = AABB::surrounding_box(&box_left, &box_right);

        BVHNode { left, right, bound }
    }

    pub fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bound.clone();
        true
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> Ordering {
        let mut box_a: AABB = AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        let mut box_b: AABB = AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));

        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            eprintln!("No bounding box in bvh_node constructor.");
        }

        if box_a.min()[axis] < box_b.min()[axis] {
            Ordering::Less
        } else if box_a.min()[axis] == box_b.min()[axis] {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }

    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (mut t_min, mut t_max) = (t_min, t_max);
        if !self.bound.hit(r, &mut t_min, &mut t_max) {
            return None;
        }

        let (mut hit_left, mut hit_right) = (false, false); 
        let mut t = t_max;
        let mut rec = None;

        if let Some(left_ptr) = &self.left {
            match left_ptr.hit(r, t_min, t_max) {
                None => {
                    hit_left = false;
                },
                Some(left_rec) => {
                    rec = Some(left_rec);
                }
            }
        }

        if let Some(right_ptr) = &self.right {
            match right_ptr.hit(r, t_min, t_max) {
                None => {
                    hit_right = false;
                }
                Some(right_ptr) => {
                    t_max = if hit_left { rec.t } else { t_max };
                    right_ptr.hit(r, t_min, t_max, rec)
                }
            }
        };

        rec
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bound.clone();
        true
    }
}
