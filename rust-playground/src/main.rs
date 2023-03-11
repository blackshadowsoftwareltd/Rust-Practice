#![deny(clippy::all)]

fn main() {
    // let variable can not be changed after initialization. it's immutable. kind of constant
    let first_name: &str = "Remon";
    let last_name: &str = "Ahammad";
    let age = 23i32; // it is same as let age: i32 = 23;
    println!("{first_name} {last_name} is {age} years old");

    //? add mut keyword to make it mutable. ex: let mut variable_name: &str = "value";
    let mut full_name: &str = "";
    let mut age: u8 = 0; // u8 is unsigned 8 bit integer. it can't be negative
    let mut male: bool = false;
    {
        // {} is a separated block bind.
        print!("{full_name} {age} {male}");
    }
    full_name = "Remon Ahammad";
    age = 23;
    male = true;
    println!("{full_name} is {age} years old & Male? {male}");

    let height: f32 = 5.9;
    println!("Height is {height} Feet");
}
