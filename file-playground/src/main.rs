#![deny(clippy::all)]
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("files/pxfuel.jpg");
    let mb = file_size_in_mb(path).unwrap();
    println!("mb : {:?}", mb);
}

fn file_size_in_mb(path: PathBuf) -> Result<f64, String> {
    if !path.exists() {
        return Err("Error : File not Exist".to_string());
    }
    let bytes = std::fs::metadata(path).unwrap().len();
    Ok((bytes as f64 / 1024.0) / 1024.0)
}
