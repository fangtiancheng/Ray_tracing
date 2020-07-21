#[allow(clippy::float_cmp)]
mod vec3;
mod point;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use vec3::Vec3;
pub use point::Point;
pub use point::Fireworks;
use imageproc::drawing::draw_line_segment_mut;
fn main() {
    paint_fireworks();
}

fn black_white_picture(){
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

fn colorful_picture(){
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 1024);
    let bar = ProgressBar::new(1024);

    for x in 0..1024 {
        for y in 0..1024 {
            let pixel = img.get_pixel_mut(x, y);
            let red = (x / 4) as u8;
            let green = (y / 4) as u8;
            let blue = (256/4) as u8;
            // println!("Red = {} Green = {} Blue = {}",red,green,blue);
            *pixel = image::Rgb([red , green, blue]);
        }
        bar.inc(1);
    }

    img.save("output/color.png").unwrap();
    bar.finish();
}

fn paint_fireworks(){
    const N :i32= 20;
    let mut img: RgbImage = ImageBuffer::new(1024, 1024);
    let bar = ProgressBar::new(N as u64);
    for n1 in 0..N {//theta
        for n2 in 0..N {//phi
            let one = Fireworks {
                r: 30.0,
                R: 100.0,
                theta: std::f64::consts::PI*n1 as f64/(N/2)as f64,
                phi: std::f64::consts::PI*n2 as f64/(N/2) as f64,
            };
            let p = one.set(40.0/180.0*std::f64::consts::PI, 40.0/180.0*std::f64::consts::PI, 1024, 1024);
            // let pixel = img.get_pixel_mut(p.x, p.y);
            // println!("{}  {}",p.x,p.y);
            // *pixel = image::Rgb([255 , 0, 0]);
            draw_line_segment_mut(&mut img, (512.0, 512.0),(p.x as f32,p.y as f32), image::Rgb([255,0,0]));
        }
        bar.inc(1);
    }

    img.save("output/fireworks.png").unwrap();
    bar.finish();
}