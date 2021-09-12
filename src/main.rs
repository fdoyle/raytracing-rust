use std::fs::File;
use std::io::Write;
use std::process::Command;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;


fn main() -> std::io::Result<()> {
    let file_name = "./image.ppm";
    let mut file = File::create(file_name)?;
    file.write_all(b"P3\n")?;
    file.write_fmt(format_args!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT))?;
    file.write_all(b"255\n")?;
    for row in (1..IMAGE_HEIGHT+1).rev() {
        for column in 0..IMAGE_WIDTH {
            let r: i32 = ((column as f64 / (IMAGE_WIDTH-1) as f64) * 255.999) as i32;
            let g: i32 = ((row as f64 / (IMAGE_HEIGHT-1) as f64) * 255.999) as i32;
            let b: i32 = (0.25 * 255.999) as i32;
            file.write_fmt(format_args!("{} {} {}\n", r, g, b))?;
        }
    }


    Command::new("open")
        .arg(file_name)
        .output();

    Ok(())
}