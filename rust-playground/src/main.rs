#![deny(clippy::all)]

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };

    println!("{}", p1 == p2);
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
