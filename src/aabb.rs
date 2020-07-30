use crate::ray::*;
use crate::utility::*;
use crate::vec3::*;
#[derive(Clone, Copy, PartialEq)]
pub struct AABB {
    pub _min: Vec3,
    pub _max: Vec3,
}
impl AABB {
    pub const fn zero() -> Self {
        return Self {
            _min: Vec3::zero(),
            _max: Vec3::zero(),
        };
    }
    pub fn new(a: Vec3, b: Vec3) -> Self {
        return Self { _min: a, _max: b };
    }
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        let small = Vec3::new(
            fmin(box0.min().x, box1.min().x),
            fmin(box0.min().y, box1.min().y),
            fmin(box0.min().z, box1.min().z),
        );
        let big = Vec3::new(
            fmin(box0.max().x, box1.max().x),
            fmin(box0.max().y, box1.max().y),
            fmin(box0.max().z, box1.max().z),
        );

        return Self {
            _min: small,
            _max: big,
        };
    }
    pub fn min(&self) -> Vec3 {
        return self._min;
    }
    pub fn max(&self) -> Vec3 {
        return self._max;
    }
    pub fn hit(&self, ray: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        for idx in 0..3 {
            let t0 = fmin(
                (self._min.get(idx) - ray.orig.get(idx)) / ray.dir.get(idx),
                (self._max.get(idx) - ray.orig.get(idx)) / ray.dir.get(idx),
            );
            let t1 = fmax(
                (self._min.get(idx) - ray.orig.get(idx)) / ray.dir.get(idx),
                (self._max.get(idx) - ray.orig.get(idx)) / ray.dir.get(idx),
            );
            tmin = fmax(t0, tmin);
            tmax = fmin(t1, tmax);
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }
}
