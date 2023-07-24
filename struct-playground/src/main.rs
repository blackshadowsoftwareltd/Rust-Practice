#![deny(clippy::all)]

fn main() {
    let foo = "Foo".to_string();
    let pair = duplicate(foo);
    println!("{pair:?}");

    let many = add_42_millions(48_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
}

fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}
