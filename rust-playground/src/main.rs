#![deny(clippy::all)]
fn main() {
    let closure = || println!("I'm a Closure!");

    call_me(closure);
    call_me(functions);
}

// Define a function which takes a generic `F` argument
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn functions() {
    println!("I'm a function")
}
