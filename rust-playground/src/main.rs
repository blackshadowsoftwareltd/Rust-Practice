#![deny(clippy::all)]

fn main() {
    let x = 5;

    println!("Initial value of x is: {x}");

    let x = x + 1;
    println!("New value of x is: {x}");
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
//? printed :
// Initial value of x is: 5
// New value of x is: 6
// The value of x in the inner scope is: 12
// The value of x is: 6
