#![deny(clippy::all)]

fn main() {
    let x: i32 = 10;
    let ref_x: &i32 = &x;
    println!("x : {}", x);
    println!("ref x : {}", ref_x);

    let mut y: i32 = 10;
    let ref_y: &mut i32 = &mut y;
    // println!("y : {:?}", y); //? cannot borrow `y` as immutable because it is also borrowed as mutable
    println!("y_ref : {}", ref_y);
}
