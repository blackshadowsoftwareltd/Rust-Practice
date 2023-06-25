#![deny(clippy::all)]

fn main() {
    color_match(Some(0));
    color_match(Some(1));
    color_match(Some(2));
}

fn color_match(color: Option<i32>) {
    if let Some(0) = color {
        println!("The color is Red!");
    } else if let Some(1) = color {
        println!("The color is Green!");
    } else if let Some(2) = color {
        println!("The color is Blue!");
    }
}
