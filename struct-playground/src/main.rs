#![deny(clippy::all)]

fn main() {
    let point = Point(0.0, 1.0, 2.0);
    println!("Point {}, {}, {}", point.0, point.1, point.2);
}
struct Point(f32, f32, f32); // ? tuple.
