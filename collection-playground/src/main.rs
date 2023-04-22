#![deny(clippy::all)]

fn main() {
    let mut age: Option<u8> = Some(23);
    match age.as_mut() {
        Some(age) => *age += 10,
        None => println!("Envalid"),
    }
    println!("Age : {}", age.unwrap()); // Age : 33

    let mut age: Option<u8> = None;
    match age.as_mut() {
        Some(age) => *age += 10,
        None => println!("Envalid"),
    }
    println!("Age : {}", age.unwrap()); // !Envalid
                                        // !thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:16:30
}
