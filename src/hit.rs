use crate::aabb::*;
use crate::material::*;
use crate::ray::Ray;
use crate::utility::*;
use crate::vec3::Vec3;
use std::cmp::Ordering;
use std::sync::Arc;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,      //碰撞点
    pub normal: Vec3, //法线方向
    pub t: f64,       //碰撞时间
    pub u: f64,
    pub v: f64,
    pub front_face: bool,           //是否从球外面射入
    pub mat_ptr: Arc<dyn Material>, //材质
}
impl HitRecord {
    pub fn new(mp: Arc<dyn Material>) -> Self {
        return Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat_ptr: mp,
        };
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = (ray.dir * outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        return HittableList {
            objects: Vec::new(),
        };
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::clone(&rec);
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = HitRecord::clone(&temp_rec);
            }
        }
        return hit_anything;
    }
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box: AABB = AABB::zero();
        let mut first_box = true;

        for object in self.objects.iter() {
            if !object.bounding_box(t0, t1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                AABB::surrounding_box(&output_box, &temp_box)
            };
            first_box = false;
        }
        return true;
    }
}

struct BVH_Node {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bvh_box: AABB,
}

impl BVH_Node {
    pub fn new(
        objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = random_in_range_i32(0, 3);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };
        let object_span = end - start;
        let mut tmp: BVH_Node;
        if object_span == 1 {
            tmp = BVH_Node {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bvh_box: AABB::new(Vec3::zero(), Vec3::zero()),
            };
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                tmp = BVH_Node {
                    left: objects[start].clone(),
                    right: objects[start + 1].clone(),
                    bvh_box: AABB::zero(),
                };
            } else {
                tmp = BVH_Node {
                    left: objects[start + 1].clone(),
                    right: objects[start].clone(),
                    bvh_box: AABB::zero(),
                };
            }
        } else {
            objects.as_mut_slice()[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            tmp = BVH_Node {
                left: Arc::new(BVH_Node::new(objects, start, mid, time0, time1)),
                right: Arc::new(BVH_Node::new(objects, mid, end, time0, time1)),
                bvh_box: AABB::zero(),
            };
        }
        let mut box_left = AABB::zero();
        let mut box_right = AABB::zero();

        if !tmp.left.bounding_box(time0, time1, &mut box_left)
            || !tmp.right.bounding_box(time0, time1, &mut box_right)
        {
            panic!("No bounding box in bvh_node constructor.");
        }

        tmp.bvh_box = AABB::surrounding_box(&box_left, &box_right);
        return tmp;
    }
}

impl Hittable for BVH_Node {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bvh_box.hit(&ray, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(&ray, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(&ray, t_min, if hit_left { rec.t } else { t_max }, rec);

        return hit_left || hit_right;
    }
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bvh_box;
        return true;
    }
}
pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    let mut box_a = AABB::zero();
    let mut box_b = AABB::zero();
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        panic!("No bounding box in bvh_node constructor.\n");
    }
    if box_a._min.x < box_b._min.x {
        return Ordering::Less;
    } else if box_a._min.x > box_b._min.x {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    let mut box_a = AABB::zero();
    let mut box_b = AABB::zero();
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        panic!("No bounding box in bvh_node constructor.\n");
    }
    if box_a._min.y < box_b._min.y {
        return Ordering::Less;
    } else if box_a._min.y > box_b._min.y {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    let mut box_a = AABB::zero();
    let mut box_b = AABB::zero();
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        panic!("No bounding box in bvh_node constructor.\n");
    }
    if box_a._min.z < box_b._min.z {
        return Ordering::Less;
    } else if box_a._min.z > box_b._min.z {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}
