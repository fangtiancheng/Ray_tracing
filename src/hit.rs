use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord{
    pub p :Vec3,//碰撞点
    pub normal: Vec3,//法线方向
    pub t : f64,//碰撞时间
    pub front_face:bool,//是否从球外面射入
}
impl HitRecord {
    pub fn clone(other:&Self)->Self{
        return Self {
            p: other.p,
            normal: other.normal,
            t: other.t,
            front_face: other.front_face,
        };
    }
    pub fn zero() -> Self {
        return Self{
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
        };
    }
}
pub trait Hittable {
    fn hit(&self,ray:&Ray,t_min : f64,t_max : f64,rec:&mut HitRecord) -> bool;
}
pub struct HittableList{
    pub objects: Vec<Box<dyn Hittable> >,
}
impl HittableList {
    pub fn zero() ->Self {
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
