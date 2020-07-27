pub use crate::vec3::*;
pub use crate::utility::*;
pub use std::sync::Arc;

pub trait Texture{
    fn value(&self,u: f64,v: f64,p: &Vec3)->Vec3;
}
pub struct SolidColor{
    pub color_value: Vec3,
}
impl SolidColor{
    pub const fn zero() -> Self{
        return Self{
            color_value: Vec3::zero(),
        };
    }
    pub fn new(color: Vec3)-> Self{
        return Self{
            color_value: color,
        };
    }
}
impl Texture for SolidColor{
    fn value(&self,u: f64,v: f64,p: &Vec3)->Vec3{
        return self.color_value;
    }
}

pub struct CheckerTexture{
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}
impl CheckerTexture{
    pub fn new(t0: Arc<dyn Texture>,t1: Arc<dyn Texture>)->Self {
        return Self{
            odd: t0, even: t1,
        };
    }
    pub fn new_by_color(c1: Vec3,c2: Vec3)-> Self{
        return Self{
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}
impl Texture for CheckerTexture{
    fn value(&self,u: f64,v: f64,p: &Vec3)->Vec3{
        let sines = (10.0*p.x).sin()*(10.0*p.y).sin()*(10.0*p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        }
        else {
            return self.even.value(u,v,p);
        }
    }
}