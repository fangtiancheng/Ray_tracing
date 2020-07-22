#[allow(clippy::float_cmp)]
mod vec3;
mod point;
mod ray;
mod sphere;
use std::thread;
use image::{ImageBuffer, RgbImage};
use imageproc::drawing::draw_line_segment_mut;
use indicatif::ProgressBar;
pub use point::Fireworks;
pub use point::Point2;
pub use vec3::Vec3;
pub use ray::Ray;
pub use sphere::Sphere;
type Point3 = Vec3;

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);
    let width = 1600;
    let height = 900;
    let dist = 1600;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let ball :Sphere = Sphere {//一个球，球的中心在(0,0,100)
        center : Vec3::new ( 0.0 , 0.0, 100.0),
        radius : 10.0,
    };
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel_mut(x, y);
            let ray = Ray{
                orig: Vec3::new(0.0,0.0,0.0),
                dir : Vec3{
                    x: (x as i32- (width/2) as i32) as f64,
                    y: (y as i32- (height/2) as i32) as f64,
                    z: dist as f64,
                }
            };
            if ball.hit_sphere(&ray) {
                *pixel = image::Rgb([255 , 0, 0]);
            }
            else {
                *pixel = image::Rgb([0 ,0 ,((1.0- y as f64/height as f64)*255.0) as u8]);
            }
            
        }
    }

    img.save("output/my_first_sphere.png").unwrap();
}

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
fn draw_fireworks_gif(){
    let thr1 = std::thread::spawn(|| {
        for n in 0..20{
            fireworks_gif(n)
        }
    }
    );
    let thr2 = std::thread::spawn(|| {
        for n in 20..40{
            fireworks_gif(n)
        }
    }
    );
    let thr3 = std::thread::spawn(|| {
        for n in 40..60{
            fireworks_gif(n)
        }
    }
    );
    let thr4 = std::thread::spawn(|| {
        for n in 60..80{
            fireworks_gif(n)
        }
    }
    );
    let thr5 = std::thread::spawn(|| {
        for n in 80..100{
            fireworks_gif(n)
        }
    }
    );
    let thr6 = std::thread::spawn(|| {
        for n in 100..120{
            fireworks_gif(n)
        }
    }
    );
    let thr7 = std::thread::spawn(|| {
        for n in 120..140{
            fireworks_gif(n)
        }
    }
    );
    
    thr1.join().unwrap();
    thr2.join().unwrap();
    thr3.join().unwrap();
    thr4.join().unwrap();
    thr5.join().unwrap();
    thr6.join().unwrap();
    thr7.join().unwrap();
    return;
}
fn fireworks_gif(t:i32){
    let vy0: f64 = -1.0;
    let vr0: f64 = 0.18;
    let g: f64 = 0.01;
    let yt: f64 = vy0* t as f64 + 0.5*g* (t*t)as f64;
    let rt: f64 = vr0* t as f64;
    paint_fireworks(yt, rt, t);
}

fn paint_fireworks(yt:f64,rt:f64,num:i32) {
    const N: i32 = 20;
    let mut img: RgbImage = ImageBuffer::new(1024, 1024);
    for n1 in 0..N {
        //theta
        for n2 in 0..N {
            //phi
            let one = Fireworks {
                r: rt,
                R: 100.0,
                y : yt,
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
            *pixel = image::Rgb([0, 255 , 0]);
        }
    }
    img.save(format!("fireworks/img{:0>3}.png",num)).unwrap();
    println!("num = {}",num);
}

fn ball_gif(){
    let thr1 = std::thread::spawn(|| {
        for n in 0..60{
            paint_ball(30.0*(n as f64/180.0*std::f64::consts::PI ).sin(),n);
        }
    }
    );
    let thr2 = std::thread::spawn(|| {
        for n in 60..120{
            paint_ball(30.0*(n as f64/180.0*std::f64::consts::PI ).sin(),n);
        }
    }
    );
    let thr3 = std::thread::spawn(|| {
        for n in 120..180{
            paint_ball(30.0*(n as f64/180.0*std::f64::consts::PI ).sin(),n);
        }
    }
    );
    let thr4 = std::thread::spawn(|| {
        for n in 180..240{
            paint_ball(30.0*(n as f64/180.0*std::f64::consts::PI ).sin(),n);
        }
    }
    );
    let thr5 = std::thread::spawn(|| {
        for n in 240..300{
            paint_ball(30.0*(n as f64/180.0*std::f64::consts::PI ).sin(),n);
        }
    }
    );
    let thr6 = std::thread::spawn(|| {
        for n in 300..360{
            paint_ball(30.0*(n as f64/180.0*std::f64::consts::PI ).sin(),n);
        }
    }
    );
    thr1.join().unwrap();
    thr2.join().unwrap();
    thr3.join().unwrap();
    thr4.join().unwrap();
    thr5.join().unwrap();
    thr6.join().unwrap();
}

fn paint_ball(yt:f64,num:u32) {
    const N: i32 = 200;
    let mut img: RgbImage = ImageBuffer::new(1024, 1024);
    for n1 in 0..N {
        //theta
        for n2 in 0..N {
            //phi
            let one = Fireworks {
                r: 30.0,
                R: 100.0,
                y : yt,
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
            *pixel = image::Rgb([(255.0*(num as f64/180.0*std::f64::consts::PI).sin()) as u8 , (255.0*(num as f64/180.0*std::f64::consts::PI+60.0).sin()) as u8 , (255.0*(num as f64/180.0*std::f64::consts::PI+120.0).sin()) as u8 ]);
        }
    }
    img.save(format!("output/gif{:0>3}.png",num)).unwrap();
    println!("num = {}",num);
}
