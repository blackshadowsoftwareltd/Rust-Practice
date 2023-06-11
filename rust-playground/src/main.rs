#![deny(clippy::all)]

use rand::prelude::*;

fn main() {
    let mut random_number = random::<u32>();
    println!("{:?}", random_number);

    let mut rng: ThreadRng = rand::thread_rng();
    random_number = rng.gen();
    println!("{:?}", random_number);
    random_number = rng.gen();
    println!("{:?}", random_number);

    let mut numbers: Vec<i32> = (0..100).collect();
    println!("{:?}", numbers);
    numbers.shuffle(&mut rng);
    println!("{:?}", numbers);
}
