#![deny(clippy::all)]

fn main() {
    let mut count = 0;

    let rusult = loop {
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("The Result is : {}", rusult);
}
