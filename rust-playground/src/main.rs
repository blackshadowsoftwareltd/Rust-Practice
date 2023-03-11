#![deny(clippy::all)]

fn main() {
    //? print example of rust lang
    println!("Hellow world");
    print!("Hello, world!");
    println!();

    let name = "Remon Ahammad";
    let age = 23;
    //? Here, {} is a placeholder which is replaced by the value of the variable after the comma. That's why we get 31 as output instead of {}
    println!("{}", age);

    println!("Age : {}", age); // {} is a placeholder

    //? Print multiple variable
    println!("Name : {}, Age : {}", name, age);

    //? specify the numbering for placeholders to print variables in different order.
    //? name is 0 index & age is 1 index
    println!("Age : {1}, Name : {0}", name, age);

    //?also use the variable names directly inside the placeholder.
    println!("Name : {name}, Age : {age}");

    //? Print Newline
    print!("Rust is fun!\nI love Rust programming.");
}
