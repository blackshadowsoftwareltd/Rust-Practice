#![deny(clippy::all)]

fn main() {
    let say_hello = |name: String| format!("Hello, {}", name);
    let say = say_hello; // ? function Pointer. store function in another function
    println!("{}", say("Remon".to_string()));

    let sum = |x: u8, y: u8| x + y;
    let sum_of_value = sum; // ? function Pointer. store function in another function
    println!("x + y = {}", sum_of_value(2, 4));

    let discount_calculate = |price: f64, parcent: f64| {
        let d = (price * parcent) / 100.0;
        (price, parcent, d, price - d)
    };

    let discount = discount_calculate; // ? function Pointer. store function in another function
    let price = discount(199.0, 10.0);

    println!(
        "Product price : {} tk, {} % discount, discount : {} tk and subtotal price is : {} tk",
        price.0, price.1, price.2, price.3
    );
}
