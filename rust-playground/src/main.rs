#![deny(clippy::all)]
use std::cell::Cell;
fn main() {
    let person = Person::new("Remon Ahammad".to_string(), 23);
    println!("name : {}", person.name); // (no need) for avoiding unused variable
    println!("new person : {:?}", person); // ? new person : Person { name: "Remon Ahammad", age: Cell { value: 23 } }

    println!("person with incremented age : {:?}", person.age_increment()); // ?person with incremented age : Person { name: "Remon Ahammad", age: Cell { value: 24 } }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: Cell<u8>, // ? The Cell will give support for the mutability of this variable where everything is immutable
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person {
            name,
            age: Cell::new(age), // ? create a Cell variable
        }
    }
    fn age_increment(&self) -> &Self {
        self.age.set(self.age.get() + 1);
        self
    }
}
