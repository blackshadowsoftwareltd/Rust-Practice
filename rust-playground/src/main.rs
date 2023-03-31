#![deny(clippy::all)]

fn main() {
    let mut name = String::from("Remon");
    println!("{}", name);
    name = take_ownership_and_give_back(name);
    println!("{}", name);
}
fn take_ownership_and_give_back(n: String) -> String {
    n + "Ahammad"
}
