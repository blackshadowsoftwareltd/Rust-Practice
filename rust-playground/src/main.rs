#![deny(clippy::all)]

fn main() {
    let pair: (i64, bool) = (10, true);
    println!("tupple practice {:?}", tupple_example(pair));
}
fn tupple_example(pair: (i64, bool)) -> (i64, i64, bool) {
    let (int_param, bool_param) = pair;
    (int_param, int_param * int_param, !bool_param)
}
