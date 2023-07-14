#![deny(clippy::all)]

use nu_ansi_term::AnsiByteStrings;
use nu_ansi_term::Color::Green;
use nu_ansi_term::{Color::Blue, Color::Cyan, Color::Fixed, Color::Red, Color::Yellow, Style};
fn main() {
    println!("This is in red: {}", Red.paint("a red string"));

    println!(
        "How about some {} and {}?",
        Style::new().bold().paint("bold"),
        Style::new().underline().paint("underline")
    );

    println!(
        "Demonstrating {} and {}!",
        Blue.bold().paint("blue bold"),
        Yellow.underline().paint("yellow underline")
    );

    println!("Yellow on blue: {}", Yellow.on(Blue).paint("wow!"));

    println!(
        "Yellow on blue: {}",
        Style::new().on(Blue).fg(Yellow).paint("yow!")
    );

    println!(
        "Also yellow on blue: {}",
        Cyan.on(Blue).fg(Yellow).paint("zow!")
    );

    println!("Text {}", Red.normal().paint("yet another red string"));

    println!(
        "Text {}",
        Style::default().paint("a completely regular string")
    );

    println!(
        "\x1b[33mHow about some {} \x1b[33mand {}?\x1b[0m",
        Style::new().reset_before_style().bold().paint("bold"),
        Style::new()
            .reset_before_style()
            .underline()
            .paint("underline")
    );

    println!("Text {}", Fixed(134).paint("A sort of light purple"));

    println!(
        "Text {}",
        Fixed(221).on(Fixed(124)).paint("Mustard in the ketchup")
    );

    println!(
        "Text {:?}",
        AnsiByteStrings(&[
            Green.paint("user data 1\n".as_bytes()),
            Green.bold().paint("user data 2\n".as_bytes()),
        ])
        .write_to(&mut std::io::stdout())
        .unwrap()
    );
}
