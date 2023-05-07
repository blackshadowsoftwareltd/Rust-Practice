#![deny(clippy::all)]

fn main() {
    let peoples: Vec<Person> = vec![
        Person {
            name: "John".to_string(),
        },
        Person {
            name: "Doe".to_string(),
        },
    ];
    peoples.run();
    peoples.walk();

    let elephants: Vec<Elephant> = vec![
        Elephant {
            name: "Elephant 1".to_string(),
        },
        Elephant {
            name: "Elephant 2".to_string(),
        },
    ];
    elephants.walk();
}
///? Elephant only Can Walk
struct Elephant {
    name: String,
}

impl CanWalk for Elephant {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}
// ? Person Can Run Can Walk
struct Person {
    name: String,
}

impl CanRun for Person {
    fn run(&self) {
        println!("{} is running", self.name);
    }
}

impl CanWalk for Person {
    fn walk(&self) {
        println!("{} is walking", self.name)
    }
}
// ? Can Run
trait CanRun {
    fn run(&self);
}

impl<T: CanRun> CanRun for Vec<T> {
    fn run(&self) {
        for item in self {
            item.run();
        }
    }
}

// ? Can Walk
trait CanWalk {
    fn walk(&self);
}

impl<T: CanWalk> CanWalk for Vec<T> {
    fn walk(&self) {
        for item in self {
            item.walk();
        }
    }
}

// print
// John is running
// Doe is running
// John is walking
// Doe is walking
// Elephant 1 is walking
// Elephant 2 is walking
