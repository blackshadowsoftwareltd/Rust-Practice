#![deny(clippy::all)]

fn main() {
    let player = Player {
        player_name: PlayerName {
            name: String::from("Lionel Messi"),
        },
        jersey: PlayerJersey {
            number: 10,
            size: 5,
        },
    };
    println!("=> Player: {:?}", player);

    let jersey = player.jersey;
    println!("=> Player Jersey: {:?}", jersey);

    let player_name = &player.player_name; // ! we must use & to borrow the value.
    println!("=> Player Name: {:?}", player_name);

    println!("=> Player: {:?}", player); // ! other wise this line will be throw error : borrow of partially moved value: `player`. partial move occurs because `player.player_name` has type `PlayerName`,

    let player_name = player.player_name.clone(); //? no need & ref if we clone it
    println!("=> Player Name: {:?}", player_name);
    println!("=> Player: {:?}", player);
}
#[derive(Debug, Clone)] // ! Copy [the trait `Copy` cannot be implemented for this type]
struct Player {
    player_name: PlayerName,
    jersey: PlayerJersey,
}

#[derive(Debug, Clone, Copy)]
struct PlayerJersey {
    number: u8,
    size: u8,
}

#[derive(Debug, Clone)] // ! Copy [the trait `Copy` cannot be implemented for this type]
struct PlayerName {
    name: String,
}
