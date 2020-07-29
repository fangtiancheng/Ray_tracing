extern crate image;
#[allow(clippy::float_cmp)]
extern crate rand;
mod aabb;
mod camera;
mod hit;
mod material;
mod perlin;
mod point;
mod ray;
mod rectangle;
mod scene;
mod sphere;
mod texture;
mod toys;
mod utility;
mod vec3;
pub use camera::Camera;
pub use hit::*;
pub use image::GenericImage;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use material::*;
pub use perlin::*;
pub use ray::Ray;
pub use scene::*;
pub use sphere::*;
pub use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
pub use texture::*;
pub use toys::naive;
pub use utility::*;
pub use vec3::Vec3;

static mut static_world: HittableList = HittableList {
    objects: Vec::new(),
};
static mut cam: Camera = Camera::zero();
fn main() {
    return;
    // Image
    const ASPECT_RATIO: f64 = 1.0 / 1.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 200;
    const MAX_DEPTH: i32 = 20;
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
        match 6 {
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
                background = Vec3::zero();
                lookfrom = Vec3::new(26.0, 15.0, -16.0);
                lookat = Vec3::new(0.0, -2.0, 0.0);
                vfov = 20.0;
            }
            4 => {
                println!("==========SIMPLE LIGHT===========");
                static_world = simple_light();
                lookfrom = Vec3::new(26.0, 3.0, 6.0);
                lookat = Vec3::new(0.0, 2.0, 0.0);
                background = Vec3::new(0.7, 0.8, 1.0);
                vfov = 20.0;
            }
            5 => {
                println!("==========Noise Texture===========");
                static_world = two_perlin_spheres();
                background = Vec3::new(0.7, 0.8, 1.0);
                lookfrom = Vec3::new(13.0, 2.0, 3.0);
                lookat = Vec3::zero();
                vfov = 20.0;
            }
            6 => {
                println!("==========Earth===========");
                static_world = earth();
                background = Vec3::new(0.7, 0.8, 1.0);
                lookfrom = Vec3::new(26.0, 3.0, 6.0);
                lookat = Vec3::zero();
                vfov = 20.0;
            }
            _ => {
                println!("==========RECTANGEL LIGHT===========");
                static_world = rectangle_light();
                background = Vec3::zero();
                lookfrom = Vec3::new(26.0, 3.0, 6.0);
                lookat = Vec3::zero();
                vfov = 20.0;
            }
        }
        cam = Camera::new(
            lookfrom,
            lookat,
            vup,
            vfov,
            ASPECT_RATIO,
            aperture,
            dist_to_focus,
            0.0,
            1.0,
        );
    }
    println!("创建世界和相机完毕！");
    // Render
    let mut mutex_img = Arc::new(Mutex::new(ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT)));
    // 多线程
    const THRNUM: i32 = 16; //线程数量
    let mut thrpool = Vec::new();
    for thi in 0..THRNUM {
        let from = (thi as f64 / THRNUM as f64 * IMAGE_HEIGHT as f64) as u32;
        let to = ((thi + 1) as f64 / THRNUM as f64 * IMAGE_HEIGHT as f64) as u32;
        let mut mutex_img = Arc::clone(&mutex_img);
        let thr = thread::spawn(move || {
            for j in from..to {
                for i in 0..IMAGE_WIDTH {
                    let mut pixel_color = Vec3::zero();
                    for s in 0..SAMPLES_PER_PIXEL {
                        //随机采样
                        let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                        let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);
                        unsafe {
                            let ray: Ray = cam.get_ray(u, v);
                            pixel_color += ray_color_static(&ray, &background, MAX_DEPTH);
                        } // unsafe
                    }
                    // Write Back
                    let mut img = mutex_img.lock().unwrap();
                    let pixel = img.get_pixel_mut(i, j);
                    write_color(pixel, &pixel_color, SAMPLES_PER_PIXEL);
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
        .save("output/cky.png")
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
fn write_color(pixel: &mut image::Rgb<u8>, pixel_color: &Vec3, samples_per_pixel: u32) {
    let r = (pixel_color.x / samples_per_pixel as f64).sqrt();
    let g = (pixel_color.y / samples_per_pixel as f64).sqrt();
    let b = (pixel_color.z / samples_per_pixel as f64).sqrt();

    *pixel = image::Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ]);
}
