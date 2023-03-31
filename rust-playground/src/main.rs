#![deny(clippy::all)]

fn main() {
    let mut name = String::from("Remon");
    println!("{}", name);
    take_ownership_and_give_back(&mut name);
    println!("{}", name);
}
// ? reference (&) not give ownership
fn take_ownership_and_give_back(n: &mut String) {
    println!("{}", n.len());
    n.push_str(" Ahammad") // push_str concat with another String
}
