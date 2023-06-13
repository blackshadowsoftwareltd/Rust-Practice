#![deny(clippy::all)]

fn main() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 100));
}
fn pick_one<T>(x: T, y: T) -> T {
    let id: u32 = std::process::id(); //? The std::process::id() function in Rust returns the identifier (ID) of the current process. It is a simple and efficient way to obtain the unique identifier for the running process.
    println!("process ID: {}", id);
    if id % 2 == 0 {
        x
    } else {
        y
    }
}
