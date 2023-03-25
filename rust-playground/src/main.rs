#![deny(clippy::all)]

fn main() {
    let list = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    //? In Rust, the iter() method creates an iterator over the elements of a collection.
    //? The enumerate() method is then called on the iterator to create a new iterator that yields a tuple of each element of the original iterator and its index.
    //? So, when you write list.iter().enumerate(), you are creating a new iterator that yields a tuple of each element of the list array and its index. The resulting iterator is then used

    for (i, v) in list.iter().enumerate() {
        print!("i : {} v : {}, ", i, v);
    }
}
