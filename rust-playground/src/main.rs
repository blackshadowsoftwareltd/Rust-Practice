#![deny(clippy::all)]

fn main() {
    let home = IPAddress::V4("12.131.12.13".to_string());
    let loopback = IPAddress::V6("25.32.24.24".to_string());
    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);
}
#[derive(Debug)]
enum IPAddress {
    V4(String),
    V6(String),
}
