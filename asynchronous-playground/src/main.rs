#![deny(clippy::all)]

use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() {
    println!("Application Started ---->");
    let get: String = get_data_from_api().await;
    println!("Get : {}", get);
    let post: bool = post_data_to_api(get).await;
    println!("Is Posted : {}", post);
}

async fn get_data_from_api() -> String {
    sleep(Duration::from_secs(2)).await;
    "Api Response after 2 second later.".to_string()
}

async fn post_data_to_api(name: String) -> bool {
    sleep(Duration::from_secs(3)).await;
    !name.is_empty()
}
