#[allow(clippy::float_cmp)]
extern crate rand;
mod toys;
mod vec3;
mod point;
mod ray;
mod sphere;
mod camera;
mod hit;
use image::{ImageBuffer, RgbImage};
// use imageproc::drawing::draw_line_segment_mut;
// use indicatif::ProgressBar;
// pub use point::Point2;
pub use vec3::Vec3;
pub use ray::Ray;
pub use hit::*;
pub use sphere::Sphere;
pub use camera::Camera;

fn main() {
    // Image
    const aspect_ratio :f64 = 16.0 / 9.0;
    const image_width :u32 = 400;
    const image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    const samples_per_pixel :u32 = 100;
    // World
    let mut world:HittableList = HittableList::zero();
    world.objects.push( Box::new( Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
    world.objects.push( Box::new( Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)));

    // Camera
    let cam :Camera = Camera::standard() ;

    // Render
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let ball :Sphere = Sphere {//一个球，球的中心在(0,0,100)
        center : Vec3::new ( 0.0 , 0.0, 100.0),
        radius : 10.0,
    };
    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color= Vec3::zero();
            for s in 0..samples_per_pixel {
                let u = (i as f64+ rand::random::<f64>())/(image_width as f64 - 1.0);
                let v = (j as f64+ rand::random::<f64>())/(image_height as f64 - 1.0);
                let ray :Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            let pixel: &mut image::Rgb<u8> = img.get_pixel_mut(i, j);
            white_color(pixel, pixel_color,samples_per_pixel);
        }
    }

    img.save("output/camera.png").unwrap();

}

fn clamp(x:f64,min:f64,max:f64)->f64{
    if x < min {return min;}
    else if x>max {return max;}
    else {return x;}
}
fn degrees_to_radians(degrees:f64)->f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

fn random_f64() -> f64{
    return rand::random::<f64>();
}

fn random_in_range(min:f64,max:f64) -> f64{
    return min + (max-min) * rand::random::<f64>();
}

fn ray_color(ray:&Ray,world:&dyn Hittable) -> Vec3 {
    let mut rec:HitRecord = HitRecord::zero();
    if world.hit(&ray,0.0,std::f64::INFINITY,& mut rec){
        return (rec.normal+Vec3::new(1.0,1.0,1.0))*0.5;
    }
    else {
        let unit_direction: Vec3 = ray.dir.unit();
        let t = 0.5*(unit_direction.y+1.0);
        return Vec3::new(1.0,1.0,1.0)*(1.0-t)+Vec3::new(0.5,0.7,1.0)*t;
    }
}

fn white_color(pixel: &mut image::Rgb<u8>, pixel_color :Vec3,samples_per_pixel: u32){
    let mut r = pixel_color.x/samples_per_pixel as f64;
    let mut g = pixel_color.y/samples_per_pixel as f64;
    let mut b = pixel_color.z/samples_per_pixel as f64;

    *pixel = image::Rgb([
        (256.0 *clamp(r, 0.0, 0.999)) as u8 ,
        (256.0 *clamp(g, 0.0, 0.999)) as u8 ,
        (256.0 *clamp(b, 0.0, 0.999)) as u8 
    ]);
    
}