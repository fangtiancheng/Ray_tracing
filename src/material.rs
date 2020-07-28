use crate::hit::*;
use crate::ray::Ray;
use crate::texture::*;
use crate::utility::*;
use crate::vec3::Vec3;
use std::sync::Arc;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self,u: f64,v: f64,p: &Vec3)->Vec3;
}
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn new(a: Arc<dyn Texture>) -> Self {
        return Self { albedo: a };
    }
    pub fn new_by_color(a: Vec3) -> Self {
        return Self {
            albedo: Arc::new(SolidColor { color_value: a }),
        };
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction, ray_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        return true;
    }
    fn emitted(&self,u: f64,v: f64,p: &Vec3)->Vec3{
        return Vec3::zero();
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Metal {
    pub fn zero() -> Self {
        return Self {
            albedo: Vec3::zero(),
            fuzz: 0.0,
        };
    }
    pub fn new(a: Vec3, f: f64) -> Self {
        return Self { albedo: a, fuzz: f };
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(ray_in.dir.unit(), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            0.0,
        );
        *attenuation = self.albedo;
        return scattered.dir * rec.normal > 0.0;
    }
    fn emitted(&self,u: f64,v: f64,p: &Vec3)->Vec3{
        return Vec3::zero();
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}
impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::ones();
        let etai_over_etat: f64;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx;
        } else {
            etai_over_etat = self.ref_idx;
        }

        let unit_direction = ray_in.dir.unit();

        let cos_theta: f64 = fmin(-unit_direction * rec.normal, 1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p, reflected, 0.0);
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random_f64() < reflect_prob {
            let reflected = reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p, reflected, 0.0);
            return true;
        }
        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p, refracted, 0.0);
        return true;
    }
    fn emitted(&self,u: f64,v: f64,p: &Vec3)->Vec3{
        return Vec3::zero();
    }
}
pub struct DiffuseLight {
    pub emit : Arc<dyn Texture>,
}
impl DiffuseLight{
    pub fn new(a: Arc<dyn Texture>)->Self{
        return Self{
            emit: a,
        };
    }
    pub fn new_by_color(c: Vec3)->Self{
        return Self{
            emit: Arc::new(SolidColor::new(c)),
        };
    }
}

impl Material for DiffuseLight{
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool{
        return false;
    }
    fn emitted(&self,u: f64,v: f64,p: &Vec3)->Vec3{
        return self.emit.value(u, v, p);
    }

}
