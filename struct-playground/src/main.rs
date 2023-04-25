#![deny(clippy::all)]

fn main() {
    let person = Person {
        first_name: "Remon".to_string(),
        last_name: "Ahammad".to_string(),
    };
    println!("Person : {:?}", person.full_name()); // Person : "Remon Ahammad"
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

trait HasName {
    // ? trait is kind of mixin in dart
    fn full_name(&self) -> String;
}

impl HasName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
