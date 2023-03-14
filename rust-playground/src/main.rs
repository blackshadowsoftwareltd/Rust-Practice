#![deny(clippy::all)]

use std::mem;

fn main() {
    //* Array. signature [T; length].
    let arr: [u8; 5] = [1, 2, 3, 4, 5]; // [data type;length]

    // All elements can be initialized to the same value.
    let arr2: [u8; 50] = [1; 50]; // [1,1,1,1,1,1,1,1,1,.....]
    println!("First element of the array: {:?}", arr);
    println!("Second element of the array: {:?}", arr2);
    println!(
        "Length First array : {} & Second array : {}",
        arr.len(),
        arr2.len()
    );
    // mem::size_of <T> () > Returns the size of the type in bytes.
    println!(
        "First Array occupies {} bytes & Second Array occupies {} bytes",
        arr.len() * mem::size_of::<u8>(),
        arr2.len() * mem::size_of::<u8>(),
    );
    // mem::size_of_val > Returns the size of the pointed-to value in bytes.
    println!(
        "First Array occupies {} bytes & Second Array occupies {} bytes",
        mem::size_of_val(&arr),
        mem::size_of_val(&arr2),
    );
    //* Slice. signature &[T]
    let _slice: &[u8];

    // Arrays can be automatically borrowed as slices.
    let slice_from_arr: &[u8] = &arr; // memory to memory copy
    println!("Slice from Array: {:?}", slice_from_arr);
}
