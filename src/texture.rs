pub use crate::perlin::*;
pub use crate::utility::*;
pub use crate::vec3::*;
pub use image::*;
pub use std::path::*;
pub use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}
pub struct SolidColor {
    pub color_value: Vec3,
}
impl SolidColor {
    pub const fn zero() -> Self {
        return Self {
            color_value: Vec3::zero(),
        };
    }
    pub fn new(color: Vec3) -> Self {
        return Self { color_value: color };
    }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        return self.color_value;
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}
impl CheckerTexture {
    pub fn new(t0: Arc<dyn Texture>, t1: Arc<dyn Texture>) -> Self {
        return Self { odd: t0, even: t1 };
    }
    pub fn new_by_color(c1: Vec3, c2: Vec3) -> Self {
        return Self {
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        };
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
}
impl NoiseTexture {
    pub fn new() -> Self {
        return Self {
            noise: Perlin::new(),
        };
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        return Vec3::ones() * self.noise.noise(p);
    }
}
pub struct ImageTexture {
    pub data: ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
}
impl ImageTexture {
    pub fn new_by_pathstr(dir: &String) -> Self {
        return Self {
            data: image::open(&Path::new(dir)).unwrap().to_rgb(),
        };
    }
    pub fn width(&self) -> u32 {
        return self.data.width();
    }
    pub fn height(&self) -> u32 {
        return self.data.height();
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i: u32 = (u * self.width() as f64) as u32;
        let mut j: u32 = (v * self.height() as f64) as u32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= self.width() {
            i = self.width() - 1;
        }
        if j >= self.height() {
            j = self.height() - 1;
        }

        const COLOR_SCALE: f64 = 1.0 / 255.0;
        let pixel = self.data.get_pixel(i, j);
        let [r, g, b] = pixel.0;
        return Vec3::new(
            r as f64 * COLOR_SCALE,
            g as f64 * COLOR_SCALE,
            b as f64 * COLOR_SCALE,
        );
    }
}
