#![deny(clippy::all)]

fn main() {
    // ? immutable
    let s = String::from("String");
    let x = &s;
    let y = &s;
    println!("{}, {}, {}", s, x, y);

    // ? mutable
    let mut z = String::from("value");
    let x = &z;
    let y = &z;
    println!("{}, {}, {}", z, x, y);

    let xy = &mut z;
    println!("{}", xy);

    // ! println!("{}, {}, {}", z, x, y); we can not use x, y variable any more.
    // ! cause z ownershit took the xy variable.
}
