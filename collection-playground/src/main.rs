#![deny(clippy::all)]

use std::collections::VecDeque;

fn main() {
    let arr: [i32; 3] = [1, 2, 3];

    let double_arr: Vec<i32> = arr.iter().map(|&x| x * 2).collect::<Vec<i32>>();
    println!("{:?}", double_arr); // [2, 4, 6]

    let double_arr: Vec<i32> = arr.iter().map(|&x| x * 2).collect::<Vec<_>>(); //? collect() only cares about what you’re collecting into, you can still use a partial type hint, _,
    println!("{:?}", double_arr); // [2, 4, 6]

    let double_arr: VecDeque<i32> = arr.iter().map(|&x| x * 2).collect::<VecDeque<i32>>();
    println!("{:?}", double_arr); // [2, 4, 6]

    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    let chars: String = arr
        .iter()
        .map(|&x| x as u8)
        .map(|x: u8| x as char)
        .collect::<String>();
    println!("{:?}", chars); //? "abcde"
}
