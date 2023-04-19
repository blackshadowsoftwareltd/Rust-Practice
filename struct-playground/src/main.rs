#![deny(clippy::all)]

fn main() {
    println!(
        "All Animals is {:?}, {:?}, {:?}",
        Animals::Dog,
        Animals::Cat,
        Animals::Rabbit
    );
    let cat: Animals = Animals::Cat;
    match cat {
        Animals::Dog => println!("{:?}", Animals::Dog),
        Animals::Cat => println!("{:?}", Animals::Cat),
        _ => println!("{:?}", Animals::Rabbit),
    }
}

#[derive(Debug)] //? to print enum directly like println!("{:?}",Animals::Cat);
enum Animals {
    Dog,
    Cat,
    Rabbit,
}
