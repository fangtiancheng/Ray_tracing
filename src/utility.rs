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

/// # 随机数f64
pub fn random_in_range_f64(min:f64,max:f64) -> f64{
    return min + (max-min) * rand::random::<f64>();
}
/// # 随机数i32
/// 返回在[min,max]中的随机i32
pub fn random_in_range_i32(min:i32,max:i32) -> i32{
    return random_in_range_f64(min as f64, max as f64+1.0) as i32;
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

pub fn random_in_unit_disk()->Vec3{
    loop{
        let p = Vec3::new(random_in_range_f64(-1.0, 1.0),random_in_range_f64(-1.0,1.0),0.0);
        if p.squared_length() >= 1.0{continue;}
        else {return p;}
    }
}

/// # 浮点数的较小值
pub fn fmin(x: f64,y: f64)->f64{
    if x<y {x} else{y}
}
/// # 浮点数的较大值
pub fn fmax(x: f64,y: f64)->f64{
    if x<y {y} else{x}
}

pub fn get_sphere_uv(p: &Vec3,u:&mut f64,v: &mut f64){
    let phi = (p.z).atan2(p.x);
    let theta = p.y.asin();
    *u = 1.0 - (phi + std::f64::consts::PI)/(2.0*std::f64::consts::PI);
    *v = (theta + std::f64::consts::PI/2.0)/std::f64::consts::PI;
}