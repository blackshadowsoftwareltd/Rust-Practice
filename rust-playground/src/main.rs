#![deny(clippy::all)]

fn main() {
    match plus_one(Some(5)) {
        Some(v) => print!("plus with 5 : {:?}", v),
        None => println!("None"),
    };
    println!("");
    match plus_one(None) {
        Some(_) => println!("Some Data"),
        None => println!("None"),
    }
    // println!("plus with None : {:?}", ;
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(v) => Some(v + 1),
        None => None,
    }
}
