#![deny(clippy::all)]

fn main() {
    process_name("Remon", print_name); // ? passed a function in the function argument
}
// ? pass a function as a parameter in a function
fn process_name(name: &str, callback: fn(&str) -> ()) {
    callback(name)
}

fn print_name(name: &str) {
    println!("Name : {}", name)
}
