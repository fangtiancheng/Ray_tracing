use crate::vec3::Vec3;
use crate::ray::*;
use crate::hit::*;
pub struct Sphere {
    pub center : Vec3,
    pub radius : f64,
}

impl Sphere{
    pub fn new(cen: Vec3,rad:f64) -> Self{
        return Sphere{
            center: cen,
            radius: rad,
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
            }
            else if root2 < t_max && root2 > t_min {
                rec.t = root2;
                rec.p = ray.at(root2);
            }

            rec.normal = (rec.p - self.center)/self.radius;
            rec.front_face = rec.normal * ray.dir < 0.0;
            if !rec.front_face {
                rec.normal = -rec.normal;
            }
            return true;
        }   
    }
}