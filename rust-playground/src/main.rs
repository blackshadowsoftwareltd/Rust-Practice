#![deny(clippy::all)]

fn main() {
    // ! store data. kind of ternary operator
    let name = if 10 == 100 {
        String::from("Remon")
    } else {
        String::from("Ahammad")
    };
    println!("The name is {}", name);

    // ! stor data when loop is iterating
    let list = [1, 2, 3, 4, 5];
    let mut count = 0;
    let result = loop {
        if list.len() == count {
            break 0;
        }
        if 10 == list[count] {
            break list[count];
        }
        count += 1;
    };
    let found = if result == 0 {
        "Not Found".to_string()
    } else {
        "Found".to_string()
    };
    println!("result : {}", found);
}
