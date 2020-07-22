use crate::vec3::Vec3;
use crate::ray::Ray;
pub struct Sphere {
    pub center : Vec3,
    pub radius : f64,
}

impl Sphere{
    pub fn hit_sphere(&self , ray : &Ray) -> bool {
        let oc: Vec3 = ray.orig - self.center;
        let a :f64 = ray.dir * ray.dir;
        let b :f64 = 2.0 * (oc * ray.dir);
        let c :f64 = oc * oc - self.radius * self.radius;
        let delta: f64 = b*b - 4.0*a*c;
        return delta >= 0.0;   
    }
}