#![deny(clippy::all)]

use std::mem;

fn main() {
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");
        mem::drop(farewell);
    };

    // ---------------------------------------
    apply(diary);

    let double = |x| 2 * x;
    let applied_to_3 = apply_to_3(double);
    println!("3 doubled : {}", applied_to_3);

    // ---------------------------------------
    let mut x = 4;
    let add_to_2 = || x += 2;
    make_twice(add_to_2);
    println!("twice : {}", x);
}

fn apply<F>(func: F)
where
    F: FnOnce(),
{
    func();
}

fn apply_to_3<F>(func: F) -> i32
where
    F: Fn(i32) -> i32,
{
    func(3)
}

fn make_twice<F>(mut func: F)
where
    F: FnMut(),
{
    func();
}
