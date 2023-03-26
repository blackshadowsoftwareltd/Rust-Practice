#![deny(clippy::all)]

fn main() {
    print_me(23, "Remon");

    let (age, name) = get_me();
    println!("My name is : {} and I am {} years old.", name, age);
}

fn print_me(x: u32, s: &str) {
    println!("My name is : {} and I am {} years old.", s, x);
}

fn get_me() -> (u32, String) {
    let first_name = "Remon";
    let last_name = "Ahammad";
    let full_name = format!("{} {}", first_name, last_name);
    (23, full_name)
}
