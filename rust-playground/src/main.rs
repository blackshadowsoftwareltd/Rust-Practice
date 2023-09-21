#![deny(clippy::all)]

use std::{fs::File, io::Read};

use anyhow::{bail, Context, Result};

fn main() {
    match read_data("src/main.rs") {
        Ok(data) => println!("Data: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_data(path: &str) -> Result<String> {
    let mut data = String::new();
    File::open(path)
        .with_context(|| format!("Failed to open file : {:?}", path.to_string()))?
        .read_to_string(&mut data)
        .context("Failed to read")?;

    if data.is_empty() {
        bail!("Data is Empty");
    }
    Ok(data)
}
