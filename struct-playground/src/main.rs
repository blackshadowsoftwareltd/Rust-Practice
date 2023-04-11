#![deny(clippy::all)]

fn main() {
    let point = Point(0.0, 1.0, 2.0);
    point.discribe();

    let mut new_point = point.twice();
    new_point.discribe();

    new_point.twice_muted();
    new_point.discribe();

    // ? reset call by Point::reset();
    new_point = Point::reset(); // ? non associated function
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

    fn twice_muted(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }

    // ? non associated function
    fn reset() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}
// * print ->
// * Point 0, 1, 2
// * Point 0, 2, 4
// * Point 0, 4, 8
// * Point 0, 0, 0
