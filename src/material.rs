use crate::ray::Ray;
use crate::hit::*;
use crate::vec3::Vec3;

pub trait Material{
    fn scatter(&self,ray_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3,scattered: &mut Ray)->bool;
}
struct Lambertian{
    pub albedo: Vec3,
}
impl Lambertian{
    
}
impl Material for Lambertian{
    fn scatter(&self,ray_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3,scattered: &mut Ray)->bool{
        let scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}
