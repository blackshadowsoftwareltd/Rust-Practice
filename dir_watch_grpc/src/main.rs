#![deny(clippy::all)]

pub mod engine;
use engine::watcher::watch::watch_dir;

fn main() {
    watch_dir("path".to_string());
}
