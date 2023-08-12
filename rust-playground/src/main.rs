#![deny(clippy::all)]
fn main() {
    let x = 5;
    let print = || println!("x : {:?}", x);

    apply(print);
}

fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}
