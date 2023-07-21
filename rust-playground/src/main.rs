#![deny(clippy::all)]

use std::fmt::Debug;
fn main() {
    let p = Point::new(5, 10.0);
    p.print_data();
}
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: f32,
}

impl<T: Debug> Point<T> {
    fn new(x: T, y: f32) -> Self {
        Self { x, y }
    }
    fn print_data(&self) {
        println!("x : {:?}, y : {:?}", self.x, self.y);
    }
}
