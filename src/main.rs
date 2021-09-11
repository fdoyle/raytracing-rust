use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() -> std::io::Result<()>  {
    let file_name = "./image.ppm";
    println!("Hello, world!");
    let mut file = File::create(file_name)?;
    file.write_all(b"Hello World updates")?;



    Command::new("open")
        .arg(file_name)
        .output();

    Ok(())

}