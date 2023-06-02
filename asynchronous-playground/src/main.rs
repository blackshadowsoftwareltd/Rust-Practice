#![deny(clippy::all)]

use futures::executor::block_on;
fn main() {
    //? block on come from the package futures.
    //? block_on help to exicute future.
    //? otherwise async won't work;
    let name: String = block_on(get_name());
    println!("Hello, {}", name)
}

async fn get_name() -> String {
    "Remon Ahammad".to_string()
}
