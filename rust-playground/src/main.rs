#![deny(clippy::all)]

fn main() {
    let v = vec![10, 20, 30];

    for x in v {
        println!("x: {x}");
    }

    // ? step_by :: Creates an iterator starting at the same point, but stepping by the given amount at each iteration.
    for i in (1..100).step_by(10) {
        println!("i : {}", i)
    }
}
