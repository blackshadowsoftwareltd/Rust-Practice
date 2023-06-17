#![deny(clippy::all)]

fn main() {
    let mut person1;
    person1 = Person::new();
    println!("{:?}", person1);
    person1.update("Remon".to_string(), 23);
    println!("{:?}", person1);

    let mut vector: Vec<Person> = vec![person1.clone()];
    person1.reset();
    println!("{:?}", person1);
    vector.push(person1);
    println!("{:?}", vector);
}

impl Person {
    fn new() -> Self {
        Person {
            name: "Your Name".to_string(),
            age: 0,
        }
    }
    fn reset(&mut self) {
        self.name = "Your Name".to_string();
        self.age = 0;
    }
    fn update(&mut self, name: String, age: u8) {
        self.name = name;
        self.age = age;
    }
}
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u8,
}
