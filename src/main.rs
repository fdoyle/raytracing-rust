pub mod vec3;
pub mod ray;

use std::fs::File;
use std::io::Write;
use std::process::Command;
use crate::vec3::{Color, Point3, Vec3};
use crate::ray::Ray;


type float = f64;
// const IMAGE_WIDTH: i32 = 256;
// const IMAGE_HEIGHT: i32 = 256;

fn main() -> std::io::Result<()> {

    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio as f32) as i32;


    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);


    let file_name = "./image.ppm";
    let mut file = File::create(file_name)?;
    file.write_all(b"P3\n")?;
    file.write_fmt(format_args!("{} {}\n", image_width, image_height))?;
    file.write_all(b"255\n")?;
    for j in (1..image_height + 1).rev() {
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = j as f64 / image_height as f64;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(r);
            pixel_color.write(&file);
        }
    }


    Command::new("open")
        .arg(file_name)
        .output()?;

    Ok(())
}


fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool {
    let oc = r.origin - center;
    let a = r.dir.dot(r.dir);
    let b = 2.0 * oc.dot(r.dir);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant > 0.0;
}