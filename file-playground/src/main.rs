#![deny(clippy::all)]
use std::fs::File;
use std::io::ErrorKind;

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
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err("File Not Found.".to_string()),
            ErrorKind::PermissionDenied => Err("Permission Denied.".to_string()),
            ErrorKind::Unsupported => Err("Unsupported.".to_string()),
            ErrorKind::Other => Err("Other".to_string()),
            _ => Err("Unknown Error".to_string()),
        },
    }
}
