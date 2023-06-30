#![deny(clippy::all)]

fn main() -> Result<(), String> {
    let result = divide(10, 2)?;
    println!("Result: {}", result);
    Ok(())
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
