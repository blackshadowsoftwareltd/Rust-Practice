use serde::{Deserialize, Serialize};

fn main() {
    let user1 = Person {
        name: String::from("John"),
        age: 30,
        height: Some(180),
        weight: None,
        gender: Some("Male".to_string()),
    };
    let json = serde_json::to_string(&user1).unwrap();
    println!("{}", json);
    let data = serde_json::from_str::<Person>(&json).unwrap();
    println!("{:?}", data.clone());
    println!("{:?}", data.gender)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person {
    name: String,
    age: u32,
    height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")] // skip this field when serializing if it is None
    weight: Option<u32>,
    #[serde(skip)] // skip this field when serializing and deserializing
    gender: Option<String>,
}
