#[allow(clippy::float_cmp)]
extern crate rand;
mod aabb;
mod camera;
mod hit;
mod material;
mod point;
mod ray;
mod scene;
mod sphere;
mod texture;
mod utility;
mod vec3;
mod perlin;
pub use camera::Camera;
pub use hit::*;
use image::{ImageBuffer, RgbImage};
use material::*;
pub use ray::Ray;
pub use scene::*;
pub use sphere::*;
pub use texture::*;
pub use utility::*;
pub use vec3::Vec3;
pub use perlin::*;
use indicatif::ProgressBar;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

static mut static_world: HittableList = HittableList {
    objects: Vec::new(),
};
static mut cam: Camera = Camera::zero();
fn main() {
    // Image
    const aspect_ratio: f64 = 16.0 / 9.0;
    const image_width: u32 = 2000;
    const image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    const samples_per_pixel: u32 = 100;
    const max_depth: i32 = 50;
    // World
    // static  world:HittableList = random_scene();

    // Camera
    let mut lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let mut lookat = Vec3::zero();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let mut aperture: f64 = 0.0;
    let mut vfov: f64 = 40.0;
    let mut background: Vec3 = Vec3::zero();
    unsafe {
        match 3 {
            1 => {
                println!("==========RANDOM SCENE==========");
                static_world = random_scene();
                aperture = 0.1;
                background = Vec3::new(0.7, 0.8, 1.0);
                vfov = 20.0;
            }
            2 => {
                println!("==========TWO SPHERE==========");
                static_world = two_spheres();
                background = Vec3::new(0.7, 0.8, 1.0);
                aperture = 0.0;
                vfov = 20.0;
            }
            3 => {
                println!("==========RANDOM SCENE WITH LIGHT==========");
                static_world = random_scene_with_light();
                aperture = 0.0;
                background = Vec3::new(0.7, 0.8, 1.0);
                lookfrom = Vec3::new(26.0, 3.0, 6.0);
                lookat = Vec3::new(0.0, 2.0, 0.0);
                vfov = 20.0;
            }
            _ => {
                println!("==========SIMPLE LIGHT===========");
                static_world = simple_light();
                lookfrom = Vec3::new(26.0, 3.0, 6.0);
                lookat = Vec3::new(0.0, 2.0, 0.0);
                background = Vec3::new(0.7, 0.8, 1.0);
                vfov = 20.0;
            }
        }
        background = Vec3::zero();
        cam = Camera::new(
            lookfrom,
            lookat,
            vup,
            vfov,
            aspect_ratio,
            aperture,
            dist_to_focus,
            0.0,
            1.0,
        );
    }
    println!("创建世界和相机完毕！");
    // Render
    let mut mutex_img = Arc::new(Mutex::new(ImageBuffer::new(image_width, image_height)));
    // 多线程
    const THRNUM: i32 = 16; //线程数量
    let mut thrpool = Vec::new();
    for thi in 0..THRNUM {
        let from = (thi as f64 / THRNUM as f64 * image_height as f64) as u32;
        let to = ((thi + 1) as f64 / THRNUM as f64 * image_height as f64) as u32;
        let mut mutex_img = Arc::clone(&mutex_img);
        let thr = thread::spawn(move || {
            for j in from..to {
                for i in 0..image_width {
                    let mut pixel_color = Vec3::zero();
                    for s in 0..samples_per_pixel {
                        //随机采样
                        let u = (i as f64 + rand::random::<f64>()) / (image_width as f64 - 1.0);
                        let v = (j as f64 + rand::random::<f64>()) / (image_height as f64 - 1.0);
                        unsafe {
                            let ray: Ray = cam.get_ray(u, v);
                            pixel_color += ray_color_static(&ray, &background, max_depth);
                        } // unsafe
                    }
                    // Write Back
                    let mut img = mutex_img.lock().unwrap();
                    let pixel = img.get_pixel_mut(i, j);
                    grey_color(pixel, &pixel_color, samples_per_pixel);
                }
                println!("j = {} in {} to {}", j, from, to);
            }
        });
        thrpool.push(thr);
        println!("创建第 {} 个线程完毕", thi);
    }
    for thr in thrpool {
        thr.join().unwrap();
    }
    mutex_img
        .lock()
        .unwrap()
        .save("output/checkered_sphere.png")
        .unwrap();
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    let mut rec: HitRecord = HitRecord::new(Arc::new(Lambertian::new_by_color(Vec3::zero())));

    if world.hit(&ray, 0.001, std::f64::INFINITY, &mut rec) {
        let mut scattered: Ray = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
        let mut attenuation: Vec3 = Vec3::zero();
        if rec
            .mat_ptr
            .scatter(ray, &rec, &mut attenuation, &mut scattered)
        {
            return Vec3::elemul(attenuation, ray_color(&scattered, world, depth - 1));
        } else {
            return Vec3::zero();
        }
    } else {
        let unit_direction: Vec3 = ray.dir.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn ray_color_static(ray: &Ray, background: &Vec3, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    let mut rec: HitRecord = HitRecord::new(Arc::new(Lambertian::new_by_color(Vec3::zero())));
    unsafe {
        if static_world.hit(&ray, 0.001, std::f64::INFINITY, &mut rec) {
            let mut scattered: Ray = Ray::zero();
            let mut attenuation: Vec3 = Vec3::zero();
            let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
            if rec
                .mat_ptr
                .scatter(ray, &rec, &mut attenuation, &mut scattered)
            {
                return emitted
                    + Vec3::elemul(
                        attenuation,
                        ray_color_static(&scattered, background, depth - 1),
                    );
            } else {
                return emitted;
            }
        } else {
            return background.clone();
        }
    } // unsafe
}
fn white_color(pixel: &mut image::Rgb<u8>, pixel_color: &Vec3, samples_per_pixel: u32) {
    let r = pixel_color.x / samples_per_pixel as f64;
    let g = pixel_color.y / samples_per_pixel as f64;
    let b = pixel_color.z / samples_per_pixel as f64;

    *pixel = image::Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ]);
}
fn grey_color(pixel: &mut image::Rgb<u8>, pixel_color: &Vec3, samples_per_pixel: u32) {
    let r = (pixel_color.x / samples_per_pixel as f64).sqrt();
    let g = (pixel_color.y / samples_per_pixel as f64).sqrt();
    let b = (pixel_color.z / samples_per_pixel as f64).sqrt();

    *pixel = image::Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ]);
}
