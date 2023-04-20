#![deny(clippy::all)]
fn main() {
    let mut values = vec![1, 2, 3, 4, 5];
    println!("{:?}", values); // [1, 2, 3, 4, 5]

    values.push(6);
    println!("{:?}", values); // [1, 2, 3, 4, 5, 6]

    values[0] = 7;
    println!("{:?}", values); // [7, 2, 3, 4, 5, 6]

    values.remove(4);
    println!("{:?}", values); // [7, 2, 3, 4, 6]

    values.pop();
    println!("{:?}", values); // [7, 2, 3, 4]

    let mut temp = vec![8, 9, 10];
    values.extend_from_slice(&temp);
    println!("values {:?}, temp {:?}", values, temp); // [7, 2, 3, 4, 8, 9, 10]   println!("{:?}", values);

    values.clear();
    println!("{:?}", values); // []

    values.extend_from_slice(&[12, 13, 14, 15, 16, 17, 18]);
    println!("{:?}", values); // [12,13,14,15,16,17,18]

    values.append(&mut temp);
    println!("values {:?}, temp {:?}", values, temp); // values [12, 13, 14, 15,16,17,18, 8, 9, 10], temp []

    if values.contains(&15) {
        println!("YES")
    } else {
        println!("NO")
    }

    println!("is values are empty? {}", values.is_empty()); // is values are empty? false

    let filtered = values
        .into_iter()
        .filter(|&x| x > 10 && x < 15)
        .collect::<Vec<_>>();
    println!("Filtered {:?}", filtered); // Filtered [12, 13, 14]

    values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let first_where = values.into_iter().find(|&x| x > 4 && x < 8);
    match first_where {
        Some(n) => println!("{:?}", n), // 5
        None => println!("NONE"),       // if not contain? => NONE
    };
}
