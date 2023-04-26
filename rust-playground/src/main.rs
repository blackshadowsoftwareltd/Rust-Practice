#![deny(clippy::all)]

fn main() {
    let five = Some(5);

    let six_option = sum_one(five);
    let six = six_option.unwrap_or(0);
    println!("{}", six); // 6

    let none_option = sum_one(None);
    let none = none_option.unwrap_or(0);
    println!("{}", none) // 0
}
fn sum_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(_) => x.map(|v| v + 1),
    }
}
