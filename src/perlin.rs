use std::sync::Arc;
use crate::utility::*;
pub struct Perlin{
    pub ranfloat: Vec<f64>,
    pub perm_x : Vec<i32>,
    pub perm_y : Vec<i32>,
    pub perm_z : Vec<i32>,
}

impl Perlin{
    const point_count: i32 = 256;
    
    pub fn new() -> Self {
        let ranfloat = Vec::new();
        for i in 0..Perlin::point_count{
            ranfloat.push(random_f64());
        }
        return Self{

        };
    }
}