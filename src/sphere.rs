use crate::aabb::*;
use crate::hit::*;
use crate::material::*;
use crate::perlin::*;
use crate::ray::*;
use crate::texture::*;
use crate::utility::*;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Vec3, rad: f64, mp: Arc<dyn Material>) -> Self {
        return Self {
            center: cen,
            radius: rad,
            mat_ptr: mp,
        };
    }
    pub fn hit_sphere(&self, ray: &Ray) -> bool {
        let oc: Vec3 = ray.orig - self.center;
        let a: f64 = ray.dir * ray.dir;
        let b: f64 = 2.0 * (oc * ray.dir);
        let c: f64 = oc * oc - self.radius * self.radius;
        let delta: f64 = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return false;
        } else {
            return true;
        }
    }
}
impl Hittable for Sphere {
    //多态
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.orig - self.center;
        let a: f64 = ray.dir * ray.dir;
        let b: f64 = 2.0 * (oc * ray.dir);
        let c: f64 = oc * oc - self.radius * self.radius;
        let delta: f64 = b * b - 4.0 * a * c;

        if delta < 0.0 {
            return false;
        } else {
            let root1 = (-b - delta.sqrt()) / (2.0 * a);
            let root2 = (-b + delta.sqrt()) / (2.0 * a);
            if root1 < t_max && root1 > t_min {
                rec.t = root1;
                rec.p = ray.at(root1);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                get_sphere_uv(
                    &((rec.p - self.center) / self.radius),
                    &mut rec.u,
                    &mut rec.v,
                );
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            } else if root2 < t_max && root2 > t_min {
                rec.t = root2;
                rec.p = ray.at(root2);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                get_sphere_uv(
                    &((rec.p - self.center) / self.radius),
                    &mut rec.u,
                    &mut rec.v,
                );
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        return false;
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        return true;
    }
}

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl MovingSphere {
    pub fn center(&self, time: f64) -> Vec3 {
        return self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0));
    }
    pub fn new(cen0: Vec3, cen1: Vec3, t0: f64, t1: f64, rad: f64, mp: Arc<dyn Material>) -> Self {
        return Self {
            center0: cen0,
            center1: cen1,
            time0: t0,
            time1: t1,
            radius: rad,
            mat_ptr: mp,
        };
    }
}
impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.orig - self.center(ray.time());
        let a: f64 = ray.dir * ray.dir;
        let b: f64 = 2.0 * (oc * ray.dir);
        let c: f64 = oc * oc - self.radius * self.radius;
        let delta: f64 = b * b - 4.0 * a * c;

        if delta < 0.0 {
            return false;
        } else {
            let root1 = (-b - delta.sqrt()) / (2.0 * a);
            let root2 = (-b + delta.sqrt()) / (2.0 * a);
            if root1 < t_max && root1 > t_min {
                rec.t = root1;
                rec.p = ray.at(root1);
                let outward_normal: Vec3 = (rec.p - self.center(ray.time())) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            } else if root2 < t_max && root2 > t_min {
                rec.t = root2;
                rec.p = ray.at(root2);
                let outward_normal: Vec3 = (rec.p - self.center(ray.time())) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        return false;
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        *output_box = AABB::surrounding_box(&box0, &box1);
        return true;
    }
}
