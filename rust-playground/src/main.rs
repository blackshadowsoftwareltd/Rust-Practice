#![deny(clippy::all)]

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    let slice = &mut arr;
    slice[0] = 10;
    slice[1] = 20;
    println!("{:?}", arr);
}
