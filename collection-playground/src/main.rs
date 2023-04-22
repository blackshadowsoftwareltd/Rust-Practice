#![deny(clippy::all)]

fn main() {
    let name: Option<String> = Some("Remon Ahammad".to_string());
    let unwrapped_name = name.expect("Name not found");
    println!("the name is : {unwrapped_name}"); // the name is : Remon Ahammad

    let name: Option<String> = None;
    let unwrapped_name = name.expect("Name is not found");
    println!("the name is : {unwrapped_name}"); // thread 'main' panicked at 'Name is not found', src/main.rs:9:31
}
