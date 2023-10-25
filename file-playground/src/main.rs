#![deny(clippy::all)]
use std::{fs::File, path::PathBuf};

fn main() {
    let path = PathBuf::from("/home/remon/Desktop")
        .join("Root")
        .join("Child")
        .join("text.txt");

    std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    File::create(path).unwrap();
}
