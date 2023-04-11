#![deny(clippy::all)]

fn main() {
    let person1 = create_person("Remon".to_string(), 23);
    println!(
        "Person 1 name {} {}, and age {}",
        person1.first_name, person1.last_name, person1.age
    );
    let person2: Person = Person {
        first_name: "MR".to_string(),
        ..person1 // ? updat syntax. It will update all properties of the struct which are not assigned before.
    };
    println!(
        "Person 2 name {} {}, and age {}",
        person2.first_name, person2.last_name, person2.age
    );
}

fn create_person(first_name: String, age: u8) -> Person {
    let last_name = "Ahammad".to_string();
    Person {
        first_name,
        last_name,
        age,
    }
}

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

// * print ->
// * Person 1 name Remon Ahammad, and age 23
// * Person 2 name MR Ahammad, and age 23
