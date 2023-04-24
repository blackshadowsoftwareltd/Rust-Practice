#![deny(clippy::all)]

fn main() {
    let full_name = full_name();
    match full_name {
        Ok(v) => println!("Full Name : {}", v), // Full Name : Remon Ahammad
        Err(_) => println!("Error"),            // Error
    }
}

fn first_name() -> Result<String, ()> {
    Ok("Remon".to_string())
}

fn last_name() -> Result<String, ()> {
    Ok("Ahammad".to_string())
}

fn full_name() -> Result<String, ()> {
    let first_name = first_name()?;
    let last_name = last_name()?;
    Ok(format!("{} {}", first_name, last_name))
}
