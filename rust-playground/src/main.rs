#![deny(clippy::all)]

fn main() {
    let mut i = 0;

    //?  While loop
    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //? For In Loop
    for e in list {
        print!("y : {}, ", e);
    }
    println!();

    for j in 0..10 {
        print!("j : {}, ", j);
    }

    println!();

    list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //? Removes the last element from a vector and returns it, or [None] if it is empty
    while let Some(y) = list.pop() {
        print!("y : {}, ", y);
    }

    //? In a situation where you would like to break or continue for one of the outer loops, you can use labels to specify which loop the break or continue statement applies to.
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                continue 'outer;
            } // Continues the loop over `x`.
            if y % 2 == 0 {
                continue 'inner;
            } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }

    //?Always alive loop
    // loop {
    //     println!("always alive loop");
    // }
}
