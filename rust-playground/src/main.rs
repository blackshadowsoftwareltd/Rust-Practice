#![deny(clippy::all)]

fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(10);

    let mut vec2 = Vec::with_capacity(vec1.len());
    vec2.push(20);
    println!("capacity {:?}", vec2.capacity());

    vec2.extend(vec1.clone().iter());
    println!(
        "vec : {:?}, capacity : {:?}, len : {:?}",
        vec2,
        vec2.capacity(),
        vec2.len()
    ); //? vec : [20, 10], capacity : 4, len : 2

    vec1 = vec2.clone();
    vec1.push(50);
    vec2.extend(vec1.iter());
    println!(
        "vec : {:?}, capacity : {:?}, len : {:?}",
        vec2,
        vec2.capacity(),
        vec2.len()
    ); //? vec : [20, 10, 20, 10, 50], capacity : 8, len : 5
}
