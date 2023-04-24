#![deny(clippy::all)]

fn main() {
    println!("Result : {}", result().is_ok()); // Result : true
    println!("Result : {}", result().is_err()); // Result : false
}

fn result() -> Result<String, ()> {
    Ok("Success".to_string())
}
