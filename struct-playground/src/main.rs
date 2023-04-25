#![deny(clippy::all)]

fn main() {
    let person = Person::new("Remon Ahammad");
    println!(
        "First Name : {}, Last Name : {}",
        person.first_name, person.last_name
    ); // First Name : Remon, Last Name : Ahammad
}

#[derive(Debug)]
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
