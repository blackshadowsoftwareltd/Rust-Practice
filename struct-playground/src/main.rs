#![deny(clippy::all)]

fn main() {
    let person: Person = Person {
        name: "Remon".to_string(),
        age: 23,
    };
    println!("My name {}, I am {} years old", person.name, person.age)
}

struct Person {
    name: String,
    age: u8,
}
