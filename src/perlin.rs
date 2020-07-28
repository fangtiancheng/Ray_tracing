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
        let mut ranfloat = Vec::new();
        for i in 0..Perlin::point_count{
            ranfloat.push(random_f64());
        }
        return Self{
            ranfloat: ranfloat,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        };
    }
    fn perlin_generate_perm() -> Vec<i32>{
        let p = Vec::new();
        for i in 0..Perlin::point_count {
            p.push(i);
        }

        return p;
    }
    fn permute(p: &mut Vec<i32>,n: i32){
        let mut i = n-1;
        while i>0 {
            let target = random_in_range_i32(0, i);
            let tmp = p [i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
            i -= 1;
        }
    }
}

