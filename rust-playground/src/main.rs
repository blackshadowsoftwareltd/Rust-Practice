#![deny(clippy::all)]
fn main() {
    let i_32: i32 = 15;
    let f_32: f32 = i_32 as f32;
    let f_64: f64 = i_32 as f64;
    println!("{} {}", f_32, f_64);

    let i_32: i32 = f_32 as i32;
    let i_64: i32 = f_32 as i32;
    println!("{} {}", i_32, i_64);

    let f_32: f64 = i_64 as f64;
    let i_32: i32 = f_64 as i32;
    println!("{} {}", f_32, i_32);
}
