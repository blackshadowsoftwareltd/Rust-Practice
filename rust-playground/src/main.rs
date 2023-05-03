#![deny(clippy::all)]

use std::ops::Deref;
fn main() {
    let count: CustomBox<i8> = CustomBox::new(10);
    println!("Custom Box count : {}", *count); // ! err: deref error. to avoid this error we need to imple deref struct for CustomBox<T> -> deref()

    let actual_value: &i8 = count.deref(); // ? getting value without pointer (with reference) using deref()
    println!("Actual value : {}", actual_value);

    let other: i8 = *(count.deref()); // ? *count == *(count.deref()) similar things
    println!("Similar to *count == *(count.deref()) : {}", other);

    // ? implicit dereference. rust understand we need i8 (actual count data is CustomBox<i8>)
    print_value(&count); // ? passing reference of value to a function by using &value
}

struct CustomBox<T> {
    value: T,
}

impl<T> CustomBox<T> {
    fn new(value: T) -> Self {
        CustomBox { value }
    }
}

// ? Deref will help to count value directly by pointer
// ? Deref == Dereference
impl<T> Deref for CustomBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn print_value(value: &i8) {
    println!("print value : {}", value);
}
