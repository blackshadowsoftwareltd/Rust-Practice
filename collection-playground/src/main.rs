#![deny(clippy::all)]

fn main() {
    let mut list = vec![1, 2, 3, 4];
    let six: Option<&u8> = list.get(5);
    match six {
        Some(v) => println!("{:?}", v),
        None => println!("No element found in index 5"),
    }
    list.append(&mut vec![5, 6, 7]);

    let six: Option<&u8> = list.get(5);
    match six {
        Some(v) => println!("{:?}", v),
        None => println!("No element found in index 5"),
    }
    for v in &mut list {
        *v += 1;
    }
    println!("{:?}", list);
}
