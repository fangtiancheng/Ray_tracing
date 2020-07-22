#[derive(Clone, Debug, PartialEq)]
pub struct Point2 {
    pub x: u32,
    pub y: u32,
}

pub struct Fireworks {
    pub r: f64, // 烟花展开半径
    pub R: f64, // 人到烟花的水平距离
    pub y: f64, // 烟花高度
    pub theta: f64,
    pub phi: f64,
}
impl Fireworks {
    pub fn set(&self, theta_x0: f64, theta_y0: f64, length_x: u32, length_y: u32) -> Point2 {
        let middle_x = (length_x / 2) as i32;
        let middle_y = (length_y / 2) as i32;
        let len: f64 = (self.R * self.R
            + (self.r * self.theta.cos()) * (self.r * self.theta.cos())
            - 2.0 * self.R * self.r * self.theta.cos() * self.phi.cos())
        .sqrt();
        return Point2 {
            x: (middle_x
                + (((self.r * self.theta.cos() * self.phi.sin() / len).asin() / theta_x0)
                    * (middle_x as f64)) as i32) as u32,
            y: (middle_y
                + ((((self.r * self.theta.sin()+self.y) / len).atan() / theta_y0) * (middle_y as f64))
                    as i32) as u32,
        };
    }
}
