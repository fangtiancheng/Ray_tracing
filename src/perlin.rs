use crate::utility::*;
use crate::vec3::*;
pub struct Perlin {
    pub ranfloat: Vec<f64>,
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut ranfloat = Vec::new();
        for _i in 0..Perlin::POINT_COUNT {
            ranfloat.push(random_f64());
        }
        return Self {
            ranfloat: ranfloat,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        };
    }
    pub fn perlin_generate_perm() -> Vec<usize> {
        let mut p = Vec::new();
        for i in 0..Perlin::POINT_COUNT {
            p.push(i);
        }
        Self::permute(&mut p, Self::POINT_COUNT);
        return p;
    }
    pub fn permute(p: &mut Vec<usize>, n: usize) {
        let mut i: usize = n - 1;
        while i > 0 {
            let target = random_in_range_i32(0, i as i32) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;

            i -= 1;
        }
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        // let u = p.x - p.x.floor();
        // let v = p.y - p.y.floor();
        // let w = p.z - p.z.floor();
        let i = ((4.0 * p.x) as usize) % Self::POINT_COUNT;
        let j = ((4.0 * p.y) as usize) % Self::POINT_COUNT;
        let k = ((4.0 * p.z) as usize) % Self::POINT_COUNT;

        return self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]];
    }
}
