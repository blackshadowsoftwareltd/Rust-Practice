#![deny(clippy::all)]
fn main() {
    let p = Point { x: 5, y: 10.0 };
    println!("p.x = {}, p.y = {}", p.x, p.y);
}
struct Point<T> {
    x: T,
    y: f32,
}
