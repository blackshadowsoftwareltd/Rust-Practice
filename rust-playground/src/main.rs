#![deny(clippy::all)]
use std::error::Error;
fn main() {
    let result: Result<String, Box<dyn Error>> = Ok("OK".to_string());
    println!("Resutl {:?}", result);

    let result: Result<String, Box<dyn Error>> = Err(Box::new(std::fmt::Error));
    println!("Resutl {:?}", result);
}
