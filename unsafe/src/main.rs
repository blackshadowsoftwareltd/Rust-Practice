fn main() {
    let mut number = 5;
    let r1 = &number as *const i32;
    let r2 = &mut number as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
