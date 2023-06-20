#![deny(clippy::all)]

use std::collections::VecDeque;

fn main() {
    let mut _vector: Vec<i32> = Vec::new();
    for i in 0..5 {
        _vector.push(i);
        println!("Vector: {:?}", _vector);
    }
    for _i in (0..5).rev() {
        _vector.pop();
        println!("Vector: {:?}", _vector);
    }

    let mut _vec_deque: VecDeque<i32> = VecDeque::new();
    for i in 0..5 {
        _vec_deque.push_front(i);
        println!("VecDeque: {:?}", _vec_deque);
    }
    for i in 5..10 {
        _vec_deque.push_back(i);
        println!("VecDeque: {:?}", _vec_deque);
    }
    for _ in 0..5 {
        _vec_deque.pop_front();
        println!("VecDeque: {:?}", _vec_deque);
    }
    for _ in (5..10).rev() {
        _vec_deque.pop_back();
        println!("VecDeque: {:?}", _vec_deque);
    }
}
