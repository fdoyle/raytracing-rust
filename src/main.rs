use std::fs::File;
use std::io::Write;
use std::process::Command;

const IMAGE_WIDTH: i32 = 2;
const IMAGE_HEIGHT: i32 = 3;


fn main() -> std::io::Result<()> {
    let file_name = "./image.ppm";
    println!("Hello, world!");
    let mut file = File::create(file_name)?;
    file.write_all(b"P3\n")?;
    file.write_all(b"3 2\n")?;
    file.write_all(b"255\n")?;
    file.write_all(b"255 0 0 0 255 0 0 0 255\n")?;
    file.write_all(b"255 255 0 255 255 255 0 0 0\n")?;


    Command::new("open")
        .arg(file_name)
        .output();

    Ok(())
}