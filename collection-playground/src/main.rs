#![deny(clippy::all)]

fn main() {
    let mut vec_collection: VecCollection = VecCollection::new();
    println!("{:?}", vec_collection); // VecCollection { list: [] }

    vec_collection.add(10);
    println!("{:?}", vec_collection); // VecCollection { list: [10] }

    let iter: std::ops::Range<i32> = 0..5;
    let c: VecCollection = VecCollection::from_iter(iter);
    println!("{:?}", c); //? VecCollection { list: [0, 1, 2, 3, 4] }
}

#[derive(Clone, Debug)]
struct VecCollection {
    list: Vec<i32>,
}

impl VecCollection {
    fn new() -> Self {
        VecCollection { list: Vec::new() }
    }
    fn add(&mut self, element: i32) {
        self.list.push(element);
    }
}

impl FromIterator<i32> for VecCollection {
    // from_iter come from std::iter::FromIterator
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut c: VecCollection = VecCollection::new();
        for i in iter {
            c.add(i);
        }
        c
    }
}
