#![deny(clippy::all)]

fn main() {
    let color = Some(Color::Red);
    color_match(color);
    let color = Some(Color::Green);
    color_match(color);
    let color = Some(Color::Blue);
    color_match(color);
}

fn color_match(color: Option<Color>) {
    if let Some(Color::Red) = color {
        println!("The color is Red!");
    } else if let Some(Color::Green) = color {
        println!("The color is Green!");
    } else if let Some(Color::Blue) = color {
        println!("The color is Blue!");
    }
}
enum Color {
    Red,
    Green,
    Blue,
}
