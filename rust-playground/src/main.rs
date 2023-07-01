#![deny(clippy::all)]

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let first = apis("Data 1".to_string()).await;
    println!("{:?}", first);
    let second = apis("Data 2".to_string()).await;
    println!("{:?}", second);
}
async fn apis(data: String) -> String {
    sleep(Duration::from_secs(2)).await;
    data
}
