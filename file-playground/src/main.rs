#![deny(clippy::all)]

use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use base64::{decode, encode};
use image::load_from_memory;
fn main() {
    let image_path = Path::new("files/pxfuel.jpg");
    is_dir_exist(image_path);
    let base64_str = convert_image_to_base64(image_path);
    println!("Base 64 String length : {}", base64_str.len());
    convert_base64_to_image(base64_str);
}

fn is_dir_exist(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).expect("Filed to create Directory");
    }
}

fn convert_image_to_base64(path: &Path) -> String {
    let mut image_file: File = File::open(path.display().to_string()).expect("File can't be open");
    let mut image_data = Vec::new();

    image_file
        .read_to_end(&mut image_data)
        .expect("Can't be read to end");

    encode(image_data)
}

fn convert_base64_to_image(base64_str: String) {
    let image_data: Vec<u8> = decode(base64_str).expect("Failed to decode base64 str");
    let image: image::DynamicImage =
        load_from_memory(&image_data).expect("Failed to load from memory");

    let mut file = File::create("files/image.png").expect("Filed to create image");
    image
        .write_to(&mut file, image::ImageOutputFormat::Png)
        .expect("Filed to write to Png");
}

// let base64_str = "iVBORw0KGgoAAAANSUhEUgAAADIA..."; // replace with your base64 string
// let data = decode(base64_str)?;

// // Load the image from the byte array
// let image = load_from_memory(&data)?;

// // Save the image to a file
// let mut file = File::create("image.png")?;
// image.write_to(&mut file, image::ImageOutputFormat::PNG)?;
