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
    rayon::scope(|s| {
        list.par_iter().for_each(|item| {
            s.spawn(|_| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(print_data(item));
            });
        });
    });
}

async fn print_data(v: &String) {
    sleep(Duration::from_secs(2));
    println!("{:?}", v)
}
