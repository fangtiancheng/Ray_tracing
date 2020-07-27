pub use crate::vec3::Vec3;
#[derive(Clone ,Debug,Copy, PartialEq)]
pub struct Ray{
    pub orig: Vec3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray{
    pub fn new(o:Vec3,d:Vec3,time:f64)->Self {
        return Self{
            orig:o,
            dir:d,
            tm: time,
        };
    }
    pub fn at(&self , t:f64) -> Vec3 {
        return self.orig+self.dir*t;
    }
    pub fn time(&self)-> f64{
        return self.tm;
    }
}