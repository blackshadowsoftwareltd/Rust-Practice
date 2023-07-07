#![deny(clippy::all)]
#[repr(u32)]
enum Bar {
    A, // 0
    B = 10000,
    C, // 10001
}

fn main() {
    println!("A: {}", Bar::A as u32); // ? A: 0
    println!("B: {}", Bar::B as u32); // ? B: 10000
    println!("C: {}", Bar::C as u32); // ? C: 10001
}
