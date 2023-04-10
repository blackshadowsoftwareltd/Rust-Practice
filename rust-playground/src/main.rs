#![deny(clippy::all)]

fn main() {
    // ? Inline function. let variable=|parameter type| return something
    // *  let say_hello:|String| -> String = |name: String| format!("Hello, {}", name);
    let say_hello = |name: String| format!("Hello, {}", name);
    println!("{}", say_hello("Remon".to_string()));

    // *  let sum:|u8, u8| -> u8 = |x: u8, y: u8| x + y;
    let sum = |x: u8, y: u8| x + y;
    println!("x + y = {}", sum(2, 4));

    // * let discount_calculate: |f64,f64| -> (f64, f64, f64, f64) = |price: f64, parcent: f64| { return }
    let discount_calculate = |price: f64, parcent: f64| {
        let d = (price * parcent) / 100.0;
        (price, parcent, d, price - d)
    };

    let price = discount_calculate(199.0, 10.0);

    println!(
        "Product price : {} tk, {} % discount, discount : {} tk and subtotal price is : {} tk",
        price.0, price.1, price.2, price.3
    );
}
