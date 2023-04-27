#![deny(clippy::all)]

use std::fs::{self, File};
use std::io::prelude::*;

fn main() {
    // ? create
    let mut _file = create_file();

    // ? write
    write_file(&mut _file, "Hello!".to_string());

    // ? read
    let data = read_file("files/test.txt".to_string());
    println!("Data : {}", data);
}

fn create_file() -> File {
    let mut _file = File::create("files/test.txt").expect("Failed to create file");
    _file
}

fn write_file(file: &mut File, message: String) -> &mut File {
    write!(file, "{}", message).expect("Failed to wirte data");
    file
}

fn read_file(path: String) -> String {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    contents
}
