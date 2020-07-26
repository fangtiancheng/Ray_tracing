use crate::vec3::Vec3;
pub fn clamp(x:f64,min:f64,max:f64)->f64{
    if x < min {return min;}
    else if x>max {return max;}
    else {return x;}
}
pub fn degrees_to_radians(degrees:f64)->f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

pub fn random_f64() -> f64{
    return rand::random::<f64>();
}

pub fn random_in_range(min:f64,max:f64) -> f64{
    return min + (max-min) * rand::random::<f64>();
}

pub fn reflect(v: Vec3,n: Vec3)->Vec3{
    return v - n*2.0*(v*n);
}

pub fn refract(uv: Vec3,n: Vec3,etai_over_etat: f64) -> Vec3 {
    let cos_theta: f64 = -(uv* n);
    let r_out_perp:Vec3 = (uv + n*cos_theta) * etai_over_etat;
    let r_out_parallel:Vec3 = - n*((1.0 - r_out_perp.squared_length()).abs().sqrt());
    return r_out_perp + r_out_parallel;
}