#![deny(clippy::all)]

fn main() {
    let mut _str: &str = "World.";
    let mut _string: String = "Hello! ".to_string();
    _string.push_str(_str); // let mut s = String::from("foo"); s.push_str("bar"); :> "foobar"
    println!("{}", _string);

    _str = &_string[..]; // You can borrow &str slices from String via & and optionally range selection.
    println!("{}", _str);
}
