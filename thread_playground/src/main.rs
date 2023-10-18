use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    println!("Main Started");
    start_thread();
    println!("Main End");
    loop {} // To keep the main thread alive
            // If main thread (process) not exist? All thread will be killed.
}

fn start_thread() {
    println!("Thread function Started");
    thread::spawn(|| {
        println!("Thread Started");
        let mut i = 0;
        loop {
            i += 1;
            println!("Thread Running {}", i);
            thread::sleep(Duration::from_millis(100));
            println!("----------------------");
            if i == 5 {
                break;
            }
        }
        println!("Thread End");
    });
    println!("Thread function end");
}
