#![deny(clippy::all)]

fn main() {
    let mut vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("vec : {:?}", vec1);

    vec1.retain(|x| x % 2 == 0);
    println!("vec : {:?}", vec1); // ? vec : [2, 4, 6, 8, 10]

    vec1 = vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9, 10, 10];
    println!("vec : {:?}", vec1); // ? vec : [1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9, 10, 10]

    vec1.dedup(); //? Remove consecutive duplicates.
    println!("vec : {:?}", vec1); // ? vec : [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
}
