#![deny(clippy::all)]
use std::io;
fn main() {
    let mut input: String = String::new();
    let mut name: String = String::new();
    let mut age: u8 = 0;

    dbg!(name, age); //? dbg! means debug.
    age = 0;
    dbg!(age);
    println!("Enter your Name : ");

    //? take user input as Sting using io::stdin().read_line(&mut input)
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    //? remove white space from input using and convert it to string then store it in name variable
    name = input.trim().to_string();

    //  ! clear input variable !! otherwise it will be make problem in next input
    input = String::new();

    println!("Enter your Age : ");

    //? take user input as Sting using io::stdin().read_line(&mut input)
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    //? match is kind of switch case in rust
    //? input.trim().parse::<u8>() convert input to u8
    match input.trim().parse::<u8>() {
        Ok(a) => age = a, //? if input is valid then store it in age variable
        Err(e) => {
            //? if input is invalid then print error and return
            println!("Invalid input,Enter number 0 to 255 {}", e);
            return;
        }
    };

    println!("You Name is : {}, And your age is {}", name, age);
}
