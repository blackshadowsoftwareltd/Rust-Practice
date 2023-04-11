#![deny(clippy::all)]

fn main() {
    let point = Point(0.0, 1.0, 2.0);
    point.discribe();
    let new_point = point.twice();
    new_point.discribe();
}
struct Point(f32, f32, f32); // ? tuple.

impl Point {
    fn discribe(&self) {
        println!("Point {}, {}, {}", self.0, self.1, self.2);
    }

    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }
}
