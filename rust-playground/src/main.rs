#![deny(clippy::all)]

fn main() {
    let mut x = 10;
    x = if x % 2 == 0 { x / 2 } else { x / 3 + 1 };
    println!("{}", x);

    let arg = std::env::args().next();
    if let Some(v) = arg {
        println!("v is : {}", v) // ? v is : target/debug/rust-playground
    } else {
        println!("Missing name?");
    }
}
