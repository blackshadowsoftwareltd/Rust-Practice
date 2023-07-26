#![deny(clippy::all)]
use std::fmt::Display;

fn main() {
    let name = "Alice";
    let name = get_name(name);
    println!("{}", name);
}

fn get_name(name: impl Display) -> impl Display {
    format!("{} is a position", name)
}
