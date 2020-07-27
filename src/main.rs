#[allow(clippy::float_cmp)]
extern crate rand;
mod vec3;
mod point;
mod utility;
mod ray;
mod sphere;
mod camera;
mod hit;
mod material;
use image::{ImageBuffer, RgbImage};
use material::*;
pub use vec3::Vec3;
pub use ray::Ray;
pub use hit::*;
pub use utility::*;
pub use sphere::Sphere;
pub use camera::Camera;
use indicatif::ProgressBar;
use std::thread;
use std::sync::Arc;

fn main() {
    // Image
    const aspect_ratio :f64 = 3.0 / 2.0;
    const image_width :u32 = 1200;
    const image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    const samples_per_pixel :u32 = 500;
    const max_depth:i32 = 50;
    // World
    let mut world:HittableList = random_scene();

    // Bar
    let bar = ProgressBar::new(image_height as u64);

    // Camera
    let lookfrom = Vec3::new(13.0,2.0,3.0);
    let lookat = Vec3::new(0.0,0.0,-0.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let dist_to_focus:f64 = 10.0;
    let aperture = 0.1;
    let cam: Camera = Camera::new(lookfrom,lookat,vup,20.0,aspect_ratio,aperture,dist_to_focus);

    // Render
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color= Vec3::zero();
            for s in 0..samples_per_pixel {//随机采样
                let u = (i as f64+ rand::random::<f64>())/(image_width as f64 - 1.0);
                let v = (j as f64+ rand::random::<f64>())/(image_height as f64 - 1.0);
                let ray :Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world,max_depth);
            }

            let pixel: &mut image::Rgb<u8> = img.get_pixel_mut(i, j);
            grey_color(pixel, &pixel_color,samples_per_pixel);
        }
        bar.inc(1);
    }

    img.save("output/camera.png").unwrap();
    bar.finish();
}
fn th(from:i32,to:i32){

}

fn ray_color(ray:&Ray,world:&dyn Hittable,depth: i32) -> Vec3 {
    if depth <= 0{
        return Vec3::zero();
    }

    let mut rec:HitRecord = HitRecord::new(Arc::new(Lambertian{
        albedo: Vec3::zero(),
    }));

    if world.hit(&ray,0.001,std::f64::INFINITY,& mut rec){
        let mut scattered: Ray = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation: Vec3 = Vec3::zero();
        if rec.mat_ptr.scatter(ray, &rec, &mut attenuation, &mut scattered) {
            return Vec3::elemul(attenuation,ray_color(&scattered, world, depth-1));
        }
        else {
            return Vec3::zero();
        }
    }
    else {
        let unit_direction: Vec3 = ray.dir.unit();
        let t = 0.5*(unit_direction.y+1.0);
        return Vec3::ones()*(1.0-t)+Vec3::new(0.5,0.7,1.0)*t;
    }
}

fn white_color(pixel: &mut image::Rgb<u8>, pixel_color :&Vec3,samples_per_pixel: u32){
    let r = pixel_color.x/samples_per_pixel as f64;
    let g = pixel_color.y/samples_per_pixel as f64;
    let b = pixel_color.z/samples_per_pixel as f64;

    *pixel = image::Rgb([
        (256.0 *clamp(r, 0.0, 0.999)) as u8 ,
        (256.0 *clamp(g, 0.0, 0.999)) as u8 ,
        (256.0 *clamp(b, 0.0, 0.999)) as u8 
    ]);
}
fn grey_color(pixel: &mut image::Rgb<u8>, pixel_color :&Vec3,samples_per_pixel: u32){
    let r = (pixel_color.x/samples_per_pixel as f64).sqrt();
    let g = (pixel_color.y/samples_per_pixel as f64).sqrt();
    let b = (pixel_color.z/samples_per_pixel as f64).sqrt();

    *pixel = image::Rgb([
        (256.0 *clamp(r, 0.0, 0.999)) as u8 ,
        (256.0 *clamp(g, 0.0, 0.999)) as u8 ,
        (256.0 *clamp(b, 0.0, 0.999)) as u8 
    ]);
}

fn random_scene() -> HittableList{
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian{
        albedo: Vec3::new(0.5,0.5,0.5),
    });
    world.objects.push(Box::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, ground_material)));
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vec3::new(a as f64 + 0.9*random_f64(),0.2,b as f64 + 0.9*random_f64());
            if (center - Vec3::new(4.0,0.4,0.0)).length() > 0.9 {
                let sphere_material : Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::elemul(Vec3::random(), Vec3::random());
                    sphere_material = Arc::new(Lambertian{
                        albedo: albedo,
                    });
                    world.objects.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
                else {
                    if choose_mat < 0.95 {
                        // metal
                        let albedo = Vec3::random_in_range(0.5, 1.0);
                        let fuzz = random_in_range(0.0, 0.5);
                        sphere_material = Arc::new(Metal{
                            albedo: albedo, fuzz: fuzz,
                        });
                        world.objects.push(Box::new(Sphere::new(center,0.2,sphere_material)));
                    }
                    else {
                        // glass
                        sphere_material = Arc::new(Dielectric{
                            ref_idx: 1.5,
                        });
                        world.objects.push(Box::new(Sphere::new(center,0.2,sphere_material)));
                    }
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric{
        ref_idx: 1.5,
    });
    world.objects.push(Box::new(Sphere::new(Vec3::new(0.0,1.0,0.0),1.0,material1)));
    
    let material2 = Arc::new(Lambertian{
        albedo: Vec3::new(0.4,0.2,0.1),
    });
    world.objects.push(Box::new(Sphere::new(Vec3::new(-4.0,1.0,0.0),1.0,material2)));

    let material3 = Arc::new(Metal{
        albedo: Vec3::new(0.7,0.6,0.5),
        fuzz: 0.0,
    });
    world.objects.push(Box::new(Sphere::new(Vec3::new(4.0,1.0,0.0),1.0,material3)));

    return world;
}