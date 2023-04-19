#![deny(clippy::all)]

fn main() {
    let circle = Shapes::Circle(10.0, 20.0);
    if let Shapes::Circle(x, y) = circle {
        println!("Circle {:?} & {:?}", x, y);
    }
    let rect = Shapes::Rectangle(
        20.0,
        40.0,
        Size {
            width: 15.0,
            height: 25.0,
        },
    );
    if let Shapes::Rectangle(x, y, Size { width, height }) = rect {
        println!("Rectangle {:?}, {:?} Size {:?} & {:?}", x, y, width, height);
    }
    println!("match ===================================================================");
    match rect {
        Shapes::Circle(x, y) => println!("Circle {:?} & {:?}", x, y),
        Shapes::Rectangle(x, y, Size { width, height }) => {
            println!("Rectangle {:?}, {:?} Size {:?} & {:?}", x, y, width, height)
        }
    }
}

#[derive(Debug)] //? to print enum directly like println!("{:?}",Animals::Cat);
enum Shapes {
    Circle(f32, f32),
    Rectangle(f32, f32, Size),
}
#[derive(Debug)] //? to print enum property directly.
struct Size {
    width: f32,
    height: f32,
}
