#![deny(clippy::all)]

fn main() {
    let x = 50;
    let y = 20;
    println!("sum of {} + {} = {}", x, y, calculate(x, y, sum));
    println!("Subtract of {} + {} = {}", x, y, calculate(x, y, subtract));
}

fn calculate(x: u8, y: u8, callback: fn(x: u8, y: u8) -> u8) -> u8 {
    callback(x, y)
}

fn sum(x: u8, y: u8) -> u8 {
    x + y
}

fn subtract(x: u8, y: u8) -> u8 {
    x - y
}
