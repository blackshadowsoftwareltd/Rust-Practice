// #![deny(clippy::all)]

use std::{any::Any, collections::HashMap};

fn main() {
    let mut map: HashMap<String, Box<dyn Any>> = HashMap::new();

    map.insert("full_name".to_string(), Box::new("John Doe".to_string()));
    map.insert("name".to_string(), Box::new("Alice"));
    map.insert("age".to_string(), Box::new(30));
    map.insert("isStudent".to_string(), Box::new(true));

    // Accessing values
    if let Some(full_name) = map.get("full_name") {
        if let Some(full_name_str) = full_name.downcast_ref::<String>() {
            println!("Full Name : {}", full_name_str);
        }
    }

    if let Some(name) = map.get("name") {
        if let Some(name_str) = name.downcast_ref::<&str>() {
            println!("Name: {}", name_str);
        }
    }

    if let Some(age) = map.get("age") {
        if let Some(age_i32) = age.downcast_ref::<i32>() {
            println!("Age: {}", age_i32);
        }
    }

    if let Some(is_student) = map.get("isStudent") {
        if let Some(is_student_bool) = is_student.downcast_ref::<bool>() {
            println!("Is Student: {}", is_student_bool);
        }
    }
}
