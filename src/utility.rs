use crate::vec3::Vec3;
/// # 折线函数
pub fn clamp(x:f64,min:f64,max:f64)->f64{
    if x < min {return min;}
    else if x>max {return max;}
    else {return x;}
}

/// # 角度化弧度
pub fn degrees_to_radians(degrees:f64)->f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

/// # 0.0 ~ 1.0之间的f64随机数
pub fn random_f64() -> f64{
    return rand::random::<f64>();
}

/// # 随机数
pub fn random_in_range(min:f64,max:f64) -> f64{
    return min + (max-min) * rand::random::<f64>();
}

/// # 反射光线矢量
pub fn reflect(v: Vec3,n: Vec3)->Vec3{
    return v - n*2.0*(v*n);
}

/// # 折射光线矢量
pub fn refract(uv: Vec3,n: Vec3,etai_over_etat: f64) -> Vec3 {
    let cos_theta: f64 = -(uv* n);
    let r_out_perp:Vec3 = (uv + n*cos_theta) * etai_over_etat;
    let r_out_parallel:Vec3 = - n*((1.0 - r_out_perp.squared_length()).abs().sqrt());
    return r_out_perp + r_out_parallel;
}
/// # 玻璃折射
/// 克里斯多弗的多项式近似
pub fn schlick(cosin: f64,ref_idx: f64)-> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;
    return r0 + (1.0-r0)*(1.0-cosin).powf(5.0);
}

/// # 浮点数的较小值
pub fn fmin(x: f64,y: f64)->f64{
    if x<y {x} else{y}
}
/// # 浮点数的较大值
pub fn fmax(x: f64,y: f64)->f64{
    if x<y {y} else{x}
}