use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord{
    pub p :Vec3,//碰撞点
    pub normal: Vec3,//法线方向
    pub t : f64,//碰撞时间
    pub front_face:bool,//是否从球外面射入
    pub mat_ptr: Arc<dyn Material>,//材质
}
impl HitRecord {
    pub fn new(mp: Arc<dyn Material>) -> Self {
        return Self{
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat_ptr: mp,
        };
    }
    pub fn set_face_normal(&mut self,ray: &Ray,outward_normal: Vec3){
        self.front_face = (ray.dir*outward_normal) < 0.0;
        if self.front_face{
            self.normal = outward_normal;
        }
        else {
            self.normal = -outward_normal;
        }
    }
}
pub trait Hittable {
    fn hit(&self,ray:&Ray,t_min : f64,t_max : f64,rec:&mut HitRecord) -> bool;
}
pub struct HittableList{
    pub objects: Vec<Box<dyn Hittable> >,
}
impl HittableList {
    pub fn new() ->Self {
        return HittableList{
            objects: Vec::new(),
        };
    }
}

impl Hittable for HittableList {
    fn hit(&self,ray:&Ray,t_min : f64,t_max : f64,rec:&mut HitRecord) -> bool{
        let mut temp_rec:HitRecord = HitRecord::clone(&rec);
        let mut hit_anything:bool = false;
        let mut closest_so_far:f64 = t_max;
        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = HitRecord::clone(&temp_rec);
            }
        }
        return hit_anything;
    }
}
