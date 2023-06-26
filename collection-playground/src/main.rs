#![deny(clippy::all)]

fn main() {
    inspect(&[1, 2, 3, 4]); //? every thing is here : 1 2 3 4
    inspect(&[1, 2, 3, 4, 5]); //? 1, every thing is ignore
}

fn inspect(list: &[i32]) {
    match list {
        [1, x, y, z] => println!("every thing is here : 1 {x} {y} {z}"),
        [1, ..] => println!("1, every thing is ignore"),
        _ => println!("..."),
    };
}
