#![deny(clippy::all)]

use rayon::prelude::*;
#[tokio::main]
async fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6, 7];
    list.par_iter_mut().for_each(|v| *v += 10);
    println!("{:?}", list);
}
