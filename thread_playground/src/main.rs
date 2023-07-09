#![deny(clippy::all)]

use std::{thread::sleep, time::Duration};

use rayon::prelude::*;
#[tokio::main]
async fn main() {
    let list: Vec<String> = vec![
        "Hello".to_string(),
        "World".to_string(),
        "This".to_string(),
        "is".to_string(),
        "a".to_string(),
        "test".to_string(),
    ];
    // do_something(&list).await;
    do_something(&list).await;
}

async fn do_something(list: &[String]) {
    list.par_iter().for_each(|x| print_data(x));
}
fn print_data(v: &String) {
    sleep(Duration::from_secs(2));
    println!("{:?}", v)
}
