#![deny(clippy::all)]

fn main() {
    let mut list: Vec<ListType> = Vec::new();
    for i in 0..10 {
        if i % 2 == 0 {
            list.push(ListType::Left(i))
        } else {
            list.push(ListType::Right(i as f32))
        }
    }
    for i in list.iter() {
        println!("{:?}", i);
    }
}
#[derive(Debug)]
enum ListType {
    Left(i32),
    Right(f32),
}
