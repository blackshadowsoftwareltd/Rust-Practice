#![deny(clippy::all)]

fn main() {
    let name: Option<String> = None;
    match name {
        Some(n) => println!("The name is : {}", n),
        None => println!("There is no name"),
    }
}
