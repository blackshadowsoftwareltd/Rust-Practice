#![deny(clippy::all)]

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat),
        Box::new(Dog {
            name: "Fido".to_string(),
        }),
    ];

    for pet in pets {
        println!("Hay {:?}", pet.name())
    }
}
trait Pet {
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat;

impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Pet for Cat {
    fn name(&self) -> String {
        String::from("Mioaw")
    }
}
