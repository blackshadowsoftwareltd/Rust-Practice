#![deny(clippy::all)]

fn main() {
    let p1 = Point {
        x: 10,
        y: 20,
        z: 30,
    };
    let p2 = Point {
        x: 20,
        y: 30,
        z: 40,
    };
    println!("Is Equal p1 & p2 {}", p1.equality(&p2))
}

struct Point {
    x: u64,
    y: u64,
    z: u64,
}
impl Point {
    fn equality(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
