#![deny(clippy::all)]

fn main() {
    let outer_value = 5;

    let closure_annotated = |i: i32| -> i32 { i + outer_value };
    let closure_inferred = |i: i32| i + outer_value;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    let one = || 1;
    println!("closure returning one: {}", one());
}
