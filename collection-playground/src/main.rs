#![deny(clippy::all)]
use std::collections::HashMap;
fn main() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("name".to_string(), "Remon Ahammad".to_string());
    map.insert("age".to_string(), "23".to_string());
    println!("map : {:?}", map); // map : {"age": "23", "name": "Remon Ahammad"}

    map.remove("age");
    println!("map : {:?}", map); // map : {"name": "Remon Ahammad"}
    if map.contains_key("age") {
        println!("Exist");
    }

    let name = map.get("name");
    match name {
        Some(name) => println!("The value of name is : {}", name),
        None => println!("the key does not exist"),
    }

    for (k, v) in &map {
        println!("K : {}, V : {}", k, v);
    }

    let entry = map.entry("name".to_string());
    println!("Entry : k : {}", entry.key()); // Entry : k : name
    match entry {
        std::collections::hash_map::Entry::Occupied(e) => {
            // ! Occupied means exist
            println!("The value of Entry is : {:?}", e.get())
        }
        std::collections::hash_map::Entry::Vacant(e) => {
            // ! Vacant means Not exist
            println!("The {:?} key does not exist ", e.key())
        }
    }

    map.entry(String::from("name"))
        .or_insert("default".to_string());
    println!("map : {:?}", map); // map : {"name": "Remon Ahammad"}

    map.entry(String::from("default_key"))
        .or_insert("default".to_string());
    println!("map : {:?}", map); // map : {"name": "Remon Ahammad", "default_key": "default"}
}
