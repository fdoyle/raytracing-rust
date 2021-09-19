pub mod vec3;

use std::fs::File;
use std::io::Write;
use std::process::Command;
use crate::vec3::Color;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;


pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f64
}

fn main() -> std::io::Result<()> {
    let file_name = "./image.ppm";
    let mut file = File::create(file_name)?;
    file.write_all(b"P3\n")?;
    file.write_fmt(format_args!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT))?;
    file.write_all(b"255\n")?;
    for row in (1..IMAGE_HEIGHT+1).rev() {
        for column in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                (row as f32) / (IMAGE_WIDTH -1) as f32,
                (column as f32) / (IMAGE_HEIGHT - 1) as f32,
                0.25);
            pixel_color.write(&file)?;
        }
    }


    Command::new("open")
        .arg(file_name)
        .output()?;

    Ok(())
}