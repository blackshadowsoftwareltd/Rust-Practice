#![deny(clippy::all)]

fn main() {
    let first_name = "Remon".to_string();
    let last_name = &first_name;
    println!("Value : {}, {}", first_name, last_name);
    greet(&first_name); // ! reference of the name
    greet(last_name); // ! already type of refered variable. so no need "&" here
    println!("Value : {}, {}", first_name, last_name);
}
fn greet(name: &String) {
    println!("Name {}", name);
}
