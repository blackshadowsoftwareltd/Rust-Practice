#![deny(clippy::all)]

fn main() {
    let mut dummy_text = String::new();
    dummy_text = String::from("World");
    dummy_text.push_str("Hello");
    println!("{:?}", dummy_text);

    let mut text = String::with_capacity(dummy_text.len());
    println!("{:?}", text.len());

    println!("{:?}", dummy_text);

    text = dummy_text;

    println!("{:?}", text);
}
