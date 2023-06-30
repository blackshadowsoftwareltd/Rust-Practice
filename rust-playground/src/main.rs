#![deny(clippy::all)]

fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    {
        let condition = !numbers.is_empty() && numbers.first().is_some();
        let first = if condition {
            numbers.first().unwrap()
        } else {
            &0
        };
        println!("{:?}", first);
    }
    numbers.push(0);
}
