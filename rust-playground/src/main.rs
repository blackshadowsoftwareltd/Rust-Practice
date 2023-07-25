#![deny(clippy::all)]

use std::sync::OnceLock;

fn main() {
    let _: &String = CELL.get_or_init(|| "Hello, World!".to_string());

    let value: Option<&String> = CELL.get();
    println!("{:?}", value);
}

pub static CELL: OnceLock<String> = OnceLock::new();
