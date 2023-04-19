#![deny(clippy::all)]
use std::f32::consts::PI;

fn main() {
    let circle = Shapes::Circle(10.0, 20.0, 30.0);

    let rect = Shapes::Rectangle(
        20.0,
        40.0,
        Size {
            width: 15.0,
            height: 25.0,
        },
    );
    println!("Area of Circle {}", circle.area());
    println!("Area of Rectangle {}", rect.area());
}
impl Shapes {
    fn area(&self) -> f32 {
        match self {
            Shapes::Rectangle(_, _, Size { width, height }) => width * height,
            Shapes::Circle(_, _, radius) => PI * radius * radius,
        }
    }
}

#[derive(Debug)] //? to print enum directly like println!("{:?}",Animals::Cat);
enum Shapes {
    Circle(f32, f32, f32),
    Rectangle(f32, f32, Size),
}
#[derive(Debug)] //? to print enum property directly.
struct Size {
    width: f32,
    height: f32,
}
