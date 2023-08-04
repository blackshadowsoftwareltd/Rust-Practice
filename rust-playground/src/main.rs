#![deny(clippy::all)]

fn main() {
    let outer_value = 5;
    //?regular function can't refer to variables in the enclosing environment
    let v = |x| x + outer_value;
    apply_with_log(v, 10);
}

fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    print!("Calling function for {:?}", input);
    func(input)
}
