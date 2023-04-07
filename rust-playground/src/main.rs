#![deny(clippy::all)]

fn main() {
    let name = "Remon".to_string();
    println!("Value : {}", name);
    greet(&name);
    println!("Value : {}", name);
    // ! Ownership does not work there cause used "&". 
    // ! "&" made the reference to the name variable. 
    // ! That means a copy of the variable is passed to the method, not Ownership.
}
fn greet(name: &String) {
    println!("Name {}", name);
}
