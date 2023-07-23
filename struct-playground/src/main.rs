#![deny(clippy::all)]

fn main() {
    let p1 = Player::default();
    let p2 = p1.clone();
    println!(
        "IS {:?}\nEqual to {:?}?The answer is {:?}",
        &p1,
        &p2,
        if p1 == p2 { "Yes" } else { "No" }
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]

struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}
