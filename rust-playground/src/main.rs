#![deny(clippy::all)]
fn main() {
    let result: Result<String, ()> = Ok("Success".to_string());
    match result {
        Ok(v) => println!("Ok {}", v),
        Err(_) => println!("Void Error"),
    }

    let result: Result<String, ()> = Err(());
    match result {
        Ok(v) => println!("Ok {}", v),
        Err(_) => println!("Void Error"),
    }
}
