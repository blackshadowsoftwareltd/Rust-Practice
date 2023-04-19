#![deny(clippy::all)]

fn main() {
    let circle = Shapes::Circle {
        radius: 80,
        center: (10, 20),
    };
    if let Shapes::Circle { radius, center } = circle {
        println!(
            "Circle Radius {:?}, Center {:?} & {:?}",
            radius, center.0, center.1
        );
    }
    let rect = Shapes::Rectanble {
        width: 50,
        height: 40,
    };
    if let Shapes::Rectanble { width, height } = rect {
        println!("Ractangle width{:?}, height {:?}", width, height);
    };
}

#[derive(Debug)] //? to print enum directly like println!("{:?}",Animals::Cat);S
enum Shapes {
    Circle { radius: i64, center: (i64, i64) },
    Rectanble { width: i64, height: i64 },
}
