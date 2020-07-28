use crate::ray::*;
use crate::utility::*;
use crate::vec3::Vec3;
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        t0: f64,
        t1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);
        let horizontal: Vec3 = u * viewport_width * focus_dist;
        let vertical: Vec3 = v * viewport_height * focus_dist;
        let origin: Vec3 = lookfrom;
        return Self {
            origin: origin,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
            time0: t0,
            time1: t1,
        };
    }
    pub const fn zero() -> Self {
        return Self {
            origin: Vec3::zero(),
            lower_left_corner: Vec3::zero(),
            horizontal: Vec3::zero(),
            vertical: Vec3::zero(),
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
            lens_radius: 0.0,
            time0: 0.0, //快门打开
            time1: 0.0, //快门关闭
        };
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        return Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
            tm: random_in_range_f64(self.time0, self.time1),
        };
    }
}
