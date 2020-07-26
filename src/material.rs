use crate::ray::Ray;
use crate::hit::*;
use crate::vec3::Vec3;
use crate::utility::*;
use std::sync::Arc;

pub trait Material{
    fn scatter(&self,ray_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3,scattered: &mut Ray)->bool;
}
pub struct Lambertian{
    pub albedo: Vec3,
}
impl Lambertian{
    fn new()->Self{
        return Self{
            albedo: Vec3::zero(),
        };
    }
}
impl Material for Lambertian{
    fn scatter(&self,ray_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3,scattered: &mut Ray)->bool{
        let scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal{
    pub albedo : Vec3,
    pub fuzz : f64,
}
impl Metal {
    pub fn zero()->Self{
        return Self{
            albedo : Vec3::zero(),
            fuzz : 0.0,
        };
    }
    pub fn new(a: Vec3, f: f64)->Self{
        return Self{
            albedo : a,
            fuzz   : f,
        };
    }
}
impl Material for Metal{
    fn scatter(&self,ray_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3,scattered: &mut Ray)->bool{
        let reflected = reflect(ray_in.dir.unit(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere()*self.fuzz);
        *attenuation = self.albedo;
        return scattered.dir*rec.normal > 0.0;
    }
}

pub struct Dielectric{
    pub ref_idx : f64,
}
impl Material for Dielectric{
    fn scatter(&self,ray_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3,scattered: &mut Ray)->bool{
        *attenuation = Vec3::ones();
        let etai_over_etat:f64;
        if rec.front_face {etai_over_etat = 1.0 / self.ref_idx;}
        else{etai_over_etat = self.ref_idx;}

        let unit_direction = ray_in.dir.unit();

        let cos_theta: f64 = fmin(-unit_direction*rec.normal,1.0);
        let sin_theta: f64 = (1.0-cos_theta*cos_theta).sqrt();
        if etai_over_etat*sin_theta >1.0 {
            let reflected = reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random_f64() < reflect_prob {
            let reflected = reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p,reflected);
            return true;
        }
        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p,refracted);
        return true;
    }
}