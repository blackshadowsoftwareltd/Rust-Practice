#![deny(clippy::all)]

fn main() {
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{:?} == {:?} ? {:?}", a, b, a.equal(&b));
    println!("{:?} != {:?} ? {:?}", a, b, a.not_equal(&b));
}

trait Equals {
    fn equal(&self, other: &Self) -> bool;

    fn not_equal(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
    fn equal(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
