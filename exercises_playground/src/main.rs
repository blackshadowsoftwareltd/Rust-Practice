#![deny(clippy::all)]

mod user;
use user::User;

fn main() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    bob.set_name("Bob New Name".to_string());
    bob.set_age(23);
    bob.set_weight(78.0);
    println!(
        "I'm {}, I am {} years old and my weight is {}",
        bob.name(),
        bob.age(),
        bob.weight()
    );
}
