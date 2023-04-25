#![deny(clippy::all)]

use std::fmt;

fn main() {
    let person = Person::new("Remon Ahammad");
    println!("{}", person); // First Name : Remon, Last Name : Ahammad
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "First Name : {}, Last Name : {}",
            self.first_name, self.last_name
        )
    }
}

struct Person {
    first_name: String,
    last_name: String,
}

trait PersonInit {
    fn new(name: &str) -> Self;
}
impl PersonInit for Person {
    fn new(name: &str) -> Self {
        let part: Vec<&str> = name.split(' ').collect();
        Person {
            first_name: part[0].to_string(),
            last_name: part[1].to_string(),
        }
    }
}
