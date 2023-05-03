#![deny(clippy::all)]
fn main() {
    let mut count: Box<i8> = Box::new(20);
    println!("count : {}", *count);

    let twice = *count * 2;
    println!("twice : {}", twice);

    *count *= 2;
    println!("twice on point of variable : {}", *count);
}
