#![deny(clippy::all)]
use intutils::addition::add;
use intutils::subtraction::sub;
fn main() {
    let x = add(10, 20);
    let y = sub(20, 10);
    println!("x : {}, y : {}", x, y);
}
