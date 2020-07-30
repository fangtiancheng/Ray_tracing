use crate::aabb::*;
use crate::hit::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct XyRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mp: Arc<dyn Material>,
}
impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        return Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp: mat,
        };
    }
}
impl Hittable for XyRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.orig.z) / ray.dir.z;
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.orig.x + t * ray.dir.x;
        let y = ray.orig.y + t * ray.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;

        let outward_normal = Vec3::new(0.0, 0.0, 0.1);
        rec.set_face_normal(ray, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = ray.at(t);
        return true;
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        );
        return true;
    }
}
