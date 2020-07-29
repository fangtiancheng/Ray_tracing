use crate::camera::*;
use crate::hit::*;
use crate::point::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;
use image::{ImageBuffer, RgbImage};
use imageproc::drawing::draw_line_segment_mut;
use indicatif::ProgressBar;
use std::thread;

// fn my_first_sphere(){
//     let x = Vec3::new(1.0, 1.0, 1.0);
//     println!("{:?}", x);
//     let width = 1600;
//     let height = 900;
//     let dist = 1600;
//     let mut img: RgbImage = ImageBuffer::new(width, height);

//     let ball :Sphere = Sphere {//一个球，球的中心在(0,0,100)
//         center : Vec3::new ( 0.0 , 0.0, 100.0),
//         radius : 10.0,
//     };
//     for x in 0..width {
//         for y in 0..height {
//             let pixel = img.get_pixel_mut(x, y);
//             let ray = Ray{
//                 orig: Vec3::new(0.0,0.0,0.0),
//                 dir : Vec3{
//                     x: (x as i32- (width/2) as i32) as f64,
//                     y: (y as i32- (height/2) as i32) as f64,
//                     z: dist as f64,
//                 }
//             };
//             if ball.hit_sphere(&ray) {
//                 *pixel = image::Rgb([255 , 0, 0]);
//             }
//             else {
//                 *pixel = image::Rgb([255 ,255 ,(( y as f64/height as f64)*255.0) as u8]);
//             }

//         }
//     }

//     img.save("output/my_first_sphere.png").unwrap();
// }

fn black_white_picture() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);

    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            let color = (x / 4) as u8;
            *pixel = image::Rgb([color, color, color]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}

fn colorful_picture() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 1024);
    let bar = ProgressBar::new(1024);

    for x in 0..1024 {
        for y in 0..1024 {
            let pixel = img.get_pixel_mut(x, y);
            let red = (x / 4) as u8;
            let green = (y / 4) as u8;
            let blue = (256 / 4) as u8;
            // println!("Red = {} Green = {} Blue = {}",red,green,blue);
            *pixel = image::Rgb([red, green, blue]);
        }
        bar.inc(1);
    }

    img.save("output/color.png").unwrap();
    bar.finish();
}
fn draw_fireworks_gif() {
    let thr1 = std::thread::spawn(|| {
        for n in 0..20 {
            fireworks_gif(n)
        }
    });
    let thr2 = std::thread::spawn(|| {
        for n in 20..40 {
            fireworks_gif(n)
        }
    });
    let thr3 = std::thread::spawn(|| {
        for n in 40..60 {
            fireworks_gif(n)
        }
    });
    let thr4 = std::thread::spawn(|| {
        for n in 60..80 {
            fireworks_gif(n)
        }
    });
    let thr5 = std::thread::spawn(|| {
        for n in 80..100 {
            fireworks_gif(n)
        }
    });
    let thr6 = std::thread::spawn(|| {
        for n in 100..120 {
            fireworks_gif(n)
        }
    });
    let thr7 = std::thread::spawn(|| {
        for n in 120..140 {
            fireworks_gif(n)
        }
    });

    thr1.join().unwrap();
    thr2.join().unwrap();
    thr3.join().unwrap();
    thr4.join().unwrap();
    thr5.join().unwrap();
    thr6.join().unwrap();
    thr7.join().unwrap();
    return;
}
fn fireworks_gif(t: i32) {
    let vy0: f64 = -1.0;
    let vr0: f64 = 0.18;
    let g: f64 = 0.01;
    let yt: f64 = vy0 * t as f64 + 0.5 * g * (t * t) as f64;
    let rt: f64 = vr0 * t as f64;
    paint_fireworks(yt, rt, t);
}

fn paint_fireworks(yt: f64, rt: f64, num: i32) {
    const N: i32 = 20;
    let mut img: RgbImage = ImageBuffer::new(1024, 1024);
    for n1 in 0..N {
        //theta
        for n2 in 0..N {
            //phi
            let one = Fireworks {
                r: rt,
                R: 100.0,
                y: yt,
                theta: std::f64::consts::PI * n1 as f64 / (N / 2) as f64,
                phi: std::f64::consts::PI * n2 as f64 / (N / 2) as f64,
            };
            let p = one.set(
                40.0 / 180.0 * std::f64::consts::PI,
                40.0 / 180.0 * std::f64::consts::PI,
                1024,
                1024,
            );
            let pixel = img.get_pixel_mut(p.x, p.y);
            *pixel = image::Rgb([0, 255, 0]);
        }
    }
    img.save(format!("fireworks/img{:0>3}.png", num)).unwrap();
    println!("num = {}", num);
}

fn ball_gif() {
    let thr1 = std::thread::spawn(|| {
        for n in 0..60 {
            paint_ball(30.0 * (n as f64 / 180.0 * std::f64::consts::PI).sin(), n);
        }
    });
    let thr2 = std::thread::spawn(|| {
        for n in 60..120 {
            paint_ball(30.0 * (n as f64 / 180.0 * std::f64::consts::PI).sin(), n);
        }
    });
    let thr3 = std::thread::spawn(|| {
        for n in 120..180 {
            paint_ball(30.0 * (n as f64 / 180.0 * std::f64::consts::PI).sin(), n);
        }
    });
    let thr4 = std::thread::spawn(|| {
        for n in 180..240 {
            paint_ball(30.0 * (n as f64 / 180.0 * std::f64::consts::PI).sin(), n);
        }
    });
    let thr5 = std::thread::spawn(|| {
        for n in 240..300 {
            paint_ball(30.0 * (n as f64 / 180.0 * std::f64::consts::PI).sin(), n);
        }
    });
    let thr6 = std::thread::spawn(|| {
        for n in 300..360 {
            paint_ball(30.0 * (n as f64 / 180.0 * std::f64::consts::PI).sin(), n);
        }
    });
    thr1.join().unwrap();
    thr2.join().unwrap();
    thr3.join().unwrap();
    thr4.join().unwrap();
    thr5.join().unwrap();
    thr6.join().unwrap();
}

fn paint_ball(yt: f64, num: u32) {
    const N: i32 = 200;
    let mut img: RgbImage = ImageBuffer::new(1024, 1024);
    for n1 in 0..N {
        //theta
        for n2 in 0..N {
            //phi
            let one = Fireworks {
                r: 30.0,
                R: 100.0,
                y: yt,
                theta: std::f64::consts::PI * n1 as f64 / (N / 2) as f64,
                phi: std::f64::consts::PI * n2 as f64 / (N / 2) as f64,
            };
            let p = one.set(
                40.0 / 180.0 * std::f64::consts::PI,
                40.0 / 180.0 * std::f64::consts::PI,
                1024,
                1024,
            );
            let pixel = img.get_pixel_mut(p.x, p.y);
            *pixel = image::Rgb([
                (255.0 * (num as f64 / 180.0 * std::f64::consts::PI).sin()) as u8,
                (255.0 * (num as f64 / 180.0 * std::f64::consts::PI + 60.0).sin()) as u8,
                (255.0 * (num as f64 / 180.0 * std::f64::consts::PI + 120.0).sin()) as u8,
            ]);
        }
    }
    img.save(format!("output/gif{:0>3}.png", num)).unwrap();
    println!("num = {}", num);
}
extern crate image;
extern crate num_complex;

pub fn naive() {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("output/naive.png").unwrap();
}
