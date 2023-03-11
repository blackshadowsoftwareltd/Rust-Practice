#![deny(clippy::all)]

fn main() {
    let x: i32 = 5;
    let y: i32 = 10;
    let z: u32 = 2;

    // Integer addition
    println!("1 + 2 = {}", 1i32 + 2i32 + x + y); // u8 + i32 not supported

    // Integer subtraction
    println!("1 - 2 = {}", 1 - 2i32); // u8 - i32 not supported

    // Short-circuiting boolean logic
    let a = false;
    let b = true;
    println!("true AND false is {}", a && b);
    println!("true OR false is {}", a || b);
    println!("NOT true is {}", !b);

    println!("1 << 5 is {}", z << 5);
    println!("0x80 >> 2 is 0 x {:x}", 0x80u32 >> z);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    println!("{}", y / x);
    println!("{}", (y * y + 3) % x)
}
