#![deny(clippy::all)]

use std::mem;

fn main() {
    let color = String::from("green");

    let print = || println!("`Color : {:?}`", color);
    // let c = color; //! cannot move out of `color` because it is borrowed
    // ! we can get access after invocking print() closure.
    print();
    let _re_borrow = &color; // ? we can get the reference of the string if we use print() bellow again
    print();
    let _color_moved = color;

    //? Mutable
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count : {:?}", count);
    };
    inc();
    // let _re_borrow = &count; // ! cannot borrow `count` as immutable because it is also borrowed as mutable
    inc();
    let _count_re_borrow = &mut count;

    //? A non-copy type.
    let movable = Box::new(5);
    let consume = || {
        println!("movable : {:?}", movable);
        mem::drop(movable);
    };
    consume();
    // consume(); // ! movable variable is droped after invocking one time.

    // ? `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);
    println!("contains(&1) {}", contains(&1));
    println!("contains(&4) {}", contains(&4));

    // println!("There're {} elements in vec", haystack.len()); // !  haystack is moved
}
