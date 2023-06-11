#![deny(clippy::all)]

fn main() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    let mut slice: &[i32] = &array;
    println!("{:?}", slice); // [1, 2, 3, 4, 5]

    println!("{:?}", &slice[0..slice.len()]); // [1, 2, 3, 4, 5]
    println!("{:?}", &slice[..slice.len()]); // [1, 2, 3, 4, 5]
    println!("{:?}", &slice[2..slice.len()]); // [3, 4, 5]
    println!("{:?}", &slice[2..]); // [3, 4, 5]

    slice = &slice[2..];
    println!("{:?}", slice);

    array = [1, 2, 3, 4, 5];
    slice = &array[..]; // To easily create a slice of the full array, we can therefore use &a[..].
    println!("{:?}", slice); // [1, 2, 3, 4, 5]
}
