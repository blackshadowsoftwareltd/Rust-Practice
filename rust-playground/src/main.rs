#![deny(clippy::all)]

use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};
pub static INFO: OnceLock<Mutex<HashSet<i32>>> = OnceLock::new();

fn main() {
    init();
    let v = INFO.get().unwrap().lock().unwrap().clone();
    println!("{:?}", v);

    INFO.get().unwrap().lock().unwrap().insert(6);
    let v = INFO.get().unwrap().lock().unwrap().clone();
    println!("{:?}", v);
}

fn init() {
    let mut info = INFO
        .get_or_init(|| {
            let mut set = HashSet::new();
            set.insert(1);
            set.insert(2);
            set.insert(3);
            Mutex::new(set)
        })
        .lock()
        .unwrap();
    info.insert(4);
    info.insert(5);
}
