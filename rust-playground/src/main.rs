#![deny(clippy::all)]

fn main() {
    let mut name = "Hellow".to_string();
    println!("value : {}", name);
    clear_txt(&mut name); // ! mutable reference has the ability to read & write permission of the variable.
    println!("value : {}", name);
}
fn clear_txt(name: &mut String) {
    name.clear(); // ! it will be clear the name variable. 
}

// outputs :
// value : Hellow
// value :