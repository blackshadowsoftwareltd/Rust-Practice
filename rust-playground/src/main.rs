#![deny(clippy::all)]

use std::iter;

fn main() {
    let five_fives_iters: iter::Take<iter::Repeat<i32>> = iter::repeat(5).take(5);
    let five_fives: Vec<i32> = Vec::from_iter(five_fives_iters);
    println!("{:?}", five_fives);
}
