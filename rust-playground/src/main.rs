#![deny(clippy::all)]
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);

    //? `into_iter()` does move `vec2` and its elements, so they cannot be used again
    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("vec2 len: {}", vec2.len());

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
    println!("First element of array2 is: {}", array2[0]);
    println!("array2 len: {}", array2.len());
}

//? Iterator::any work look like
pub trait Iterator {
    type Item; //? The type being iterated over.

    // fn any_z<F: FnMut(Self::Item)>(&mut self, f: F) -> bool;
    fn any<F>(&mut self, f: F) -> bool
    //? `any` takes `&mut self` meaning the caller may be borrowed  and modified, but not consumed.
    where
        F: FnMut(Self::Item) -> bool;
}
