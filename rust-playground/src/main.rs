#![deny(clippy::all)]

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 3u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (3u64, -1i8), -2i16);
    println!("tuple of tuples : {:?}", tuple_of_tuples);
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "Hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?} {:?} {:?} {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix.0);
}
struct Matrix(f32, f32, f32, f32);
