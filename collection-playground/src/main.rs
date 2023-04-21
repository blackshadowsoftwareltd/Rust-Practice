#![deny(clippy::all)]
use std::collections::HashMap;

fn main() {
    let user1 = Person {
        name: "Remon Ahammad".to_string(),
        age: 23,
    };
    println!("name : {}, age : {}", user1.name, user1.age);

    let mut map: HashMap<String, Person> = HashMap::new();
    map.insert("person".to_string(), user1);
    println!("Map : {:?}", map); // Map : {"person": Person { name: "Remon Ahammad", age: 23 }}
}
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
