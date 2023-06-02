#![deny(clippy::all)]

use workspace::addition_mod::add;
use workspace::subtraction_mod::sub;
fn main() {
    let _sum = add(10, 20);
    println!("{}", _sum);
    let _sub = sub(20, 10);
    println!("{}", _sub)
}
