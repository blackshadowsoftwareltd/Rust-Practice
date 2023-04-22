#![deny(clippy::all)]

fn main() {
    let mut name: Option<String> = None;
    if name.is_none() {
        println!("Name is None")
    }

    name = Some("Hello".to_string());
    if name.is_some() {
        println!("Name exist")
    }
}
