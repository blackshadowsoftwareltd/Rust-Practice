#![deny(clippy::all)]

fn main() {
    let both_int = Point { x: 2, y: 4 };
    let both_float = Point { x: 1.2, y: 3.4 };
    let int_float = Point { x: 5, y: 10.8 };
    println!(
        "boath int : {:?}, both float : {:?}, int & float : {:?}",
        both_int, both_float, int_float
    );
}

#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y,
}
