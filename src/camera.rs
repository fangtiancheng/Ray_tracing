use crate::vec3::Vec3;
use crate::ray::*;
pub struct Camera{
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    /// 创造一个标准16:9照相机
    pub fn standard() -> Self{
        let aspect_ratio:f64 = 16.0 / 9.0;
        let viewport_height:f64 = 2.0;
        let viewport_width:f64 = aspect_ratio * viewport_height;
        let focal_length:f64 = 1.0;
        // lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);
        return Self {
            origin: Vec3::zero(),
            lower_left_corner: Vec3::new(-viewport_width/2.0,- viewport_height/2.0,-focal_length),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical : Vec3::new(0.0, viewport_height, 0.0),
        };
    }
    pub fn get_ray(&self,u:f64,v:f64) -> Ray{
        return Ray{
            orig: self.origin,
            dir: self.lower_left_corner+self.horizontal*u+self.vertical*v-self.origin,
        };
    }

}