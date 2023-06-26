#![deny(clippy::all)]

fn main() {
    let _foo = Foo { x: (10, 20), y: 30 };

    match _foo {
        Foo { x: (10, b), y } => println!("x : (10, {}), {}", b, y),
        // Foo { y, x: i } => println!("y : {}, x : {:?}", y, i),
        Foo { y, .. } => println!("Ingnored x. y : {}", y),
    }
}

struct Foo {
    x: (i32, i32),
    y: i32,
}
