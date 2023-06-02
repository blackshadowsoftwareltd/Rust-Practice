#![deny(clippy::all)]

use futures::Future;
use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() {
    println!("Application Started ---->");
    let get: String = get_data_from_api().await;
    println!("Get : {}", get);
    let post: bool = post_data_to_api(get).await;
    println!("Is Posted : {}", post);
}

fn get_data_from_api() -> impl Future<Output = String> {
    //? impl Future<Output=returnType> will be able to use a fn without async keyword before function
    //? to sue async feature. we have to use async block.
    async {
        sleep(Duration::from_secs(2)).await;
        "Api Response after 2 second later.".to_string()
    }
}

fn post_data_to_api(name: String) -> impl Future<Output = bool> {
    //! without move keyword this fn won't run. cause the name variable borrowed by the async block.
    //! so we should not use this async block if we need to use varible which is not owned by this fn.
    async move {
        sleep(Duration::from_secs(3)).await;
        !name.is_empty()
    }
}
