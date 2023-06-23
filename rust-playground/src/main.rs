#![deny(clippy::all)]

fn main() {
    let x: u8 = 5;
    match x {
        0 => println!("Zero"),
        1 | 2 => println!("One or Two"),
        3..=5 => println!("3 to 5"),
        _ => println!("Default case"),
    }

    let numbers = (0, 1, 2, 3, 4, 5);
    let (first, .., last) = numbers;
    println!("First ? {} & Last ? {}", first, last);
}
