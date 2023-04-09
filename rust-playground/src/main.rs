#![deny(clippy::all)]

fn main() {
    let mut s = String::from("hello");

    //? cannot borrow `s` as mutable more than once at a time
    // ! let r1 = &mut s;
    // ! let r2 = &mut s;

    //? so we can create bloc of code for each code
    {
        let r1 = &s;
        println!("{}", r1)
    }
    {
        let r2 = &mut s;
        println!("{}", r2)
    }
}
