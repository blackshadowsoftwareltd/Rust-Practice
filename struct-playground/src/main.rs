#![deny(clippy::all)]

fn main() {
    let point = Point(0.0, 1.0, 2.0);
    point.discribe(); // ? print Point tuple another way by impl
}
struct Point(f32, f32, f32); // ? tuple.

// ? imple self
impl Point {
    fn discribe(&self) {
        println!("Point {}, {}, {}", self.0, self.1, self.2);
    }
}
