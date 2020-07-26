use crate::vec3::Vec3;
use crate::ray::*;
use crate::hit::*;
use std::sync::Arc;
use crate::material::*;
pub struct Sphere {
    pub center : Vec3,
    pub radius : f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere{
    pub fn new(cen: Vec3, rad:f64, mp: Arc<dyn Material>) -> Self{
        return Sphere{
            center : cen,
            radius : rad,
            mat_ptr: mp,
        };
    }
    pub fn hit_sphere(&self , ray : &Ray) -> bool {
        let oc: Vec3 = ray.orig - self.center;
        let a :f64 = ray.dir * ray.dir;
        let b :f64 = 2.0 * (oc * ray.dir);
        let c :f64 = oc * oc - self.radius * self.radius;
        let delta: f64 = b*b - 4.0*a*c;
        if delta < 0.0 { return false;}
        else { return true;}   
    }
}
impl Hittable for Sphere{//多态
    fn hit(&self , ray : &Ray,t_min : f64 ,t_max : f64,rec:&mut HitRecord) -> bool {
        let oc: Vec3 = ray.orig - self.center;
        let a :f64 = ray.dir * ray.dir;
        let b :f64 = 2.0 * (oc * ray.dir);
        let c :f64 = oc * oc - self.radius * self.radius;
        let delta: f64 = b*b - 4.0*a*c;
        
        if delta < 0.0 { return false;}
        else {
            let root1 = (-b-delta.sqrt())/(2.0*a);
            let root2 = (-b+delta.sqrt())/(2.0*a);
            if root1 < t_max && root1 > t_min {
                rec.t = root1;
                rec.p = ray.at(root1);
                let outward_normal:Vec3 = (rec.p - self.center)/self.radius;
                rec.set_face_normal(ray, outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            else if root2 < t_max && root2 > t_min {
                rec.t = root2;
                rec.p = ray.at(root2);
                let outward_normal:Vec3 = (rec.p - self.center)/self.radius;
                rec.set_face_normal(ray, outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        return false;
    }
}