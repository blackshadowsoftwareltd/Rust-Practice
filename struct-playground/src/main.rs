#![deny(clippy::all)]

fn main() {
    let circle = Shapes::Circle {
        radius: 80,
        center: (10, 20),
    };
    let rect = Shapes::Rectangle {
        width: 50,
        height: 40,
    };
    match circle {
        Shapes::Circle { radius, center } => println!(
            "Circle Radius {:?}, Center {:?} & {:?}",
            radius, center.0, center.1
        ),
        Shapes::Rectangle { width, height } => {
            println!("Ractangle width{:?}, height {:?}", width, height)
        }
    }

    match rect {
        Shapes::Rectangle { width, height } => {
            println!("Ractangle width{:?}, height {:?}", width, height)
        }
        Shapes::Circle { radius, center } => println!(
            "Circle Radius {:?}, Center {:?} & {:?}",
            radius, center.0, center.1
        ),
    }
}

#[derive(Debug)] //? to print enum directly like println!("{:?}",Animals::Cat);S
enum Shapes {
    Circle { radius: i64, center: (i64, i64) },
    Rectangle { width: i64, height: i64 },
}
