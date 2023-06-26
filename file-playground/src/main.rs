#![deny(clippy::all)]
use std::fs::File;

fn main() {
    let result = open_file();
    if let Ok(_file) = result {
        println!("File is exist")
    } else if let Err(message) = result {
        println!("!!! Error : {}", message);
    }
}

fn open_file() -> Result<File, String> {
    let open = File::open("file.jpeg");
    match open {
        Ok(file) => Ok(file),
        Err(e) => Err(e.to_string()),
    }
}
