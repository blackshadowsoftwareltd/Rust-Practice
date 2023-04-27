#![deny(clippy::all)]

use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let file_path = Path::new("files/test.txt");
    is_dir_exist(&file_path);

    // ? create
    let mut _file = create_file(&file_path);

    // ? write
    write_file(&mut _file, "Hello!".to_string());

    // ? read
    let data = read_file(&file_path);
    println!("Data : {}", data);

    // ? delete
    remove_file(&file_path);
}

fn is_dir_exist(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).expect("Filed to create Directory");
    }
}

fn create_file(path: &Path) -> File {
    let mut _file = File::create(path.display().to_string()).expect("Failed to create file");
    _file
}

fn write_file(file: &mut File, message: String) -> &mut File {
    write!(file, "{}", message).expect("Failed to wirte data");
    file
}

fn read_file(path: &Path) -> String {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    contents
}

fn remove_file(path: &Path) {
    match fs::remove_file(path.display().to_string()) {
        Ok(()) => println!("File deleted successfully!"),
        Err(error) => println!("Error deleting file: {:?}", error),
    }
}
