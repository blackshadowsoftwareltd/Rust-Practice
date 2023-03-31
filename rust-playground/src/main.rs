#![deny(clippy::all)]

fn main() {
    let s = String::from("Hello");
    println!("1 {}", s);
    print_string(s); // ! method took the ownership of the s variable.

    // ! println!("3 {}", s); there is an error.
    // !caouse ownership is changed to method. to fix this issue =>

    let s = String::from("Hello World");
    print_string(s.clone()); // we can pass a copy of the variable or
    print_string_by_reference(&s); // or we can pass the reference to the variable like that
    println!("-- 1 : {}", s);
}
fn print_string(s: String) {
    println!("2 {}", s);
}
fn print_string_by_reference(s: &String) {
    println!("2 {}", s);
}
