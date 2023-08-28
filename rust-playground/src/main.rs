#![deny(clippy::all)]
use chrono::prelude::*;
fn main() {
    let utc: DateTime<Utc> = Utc::now();
    println!("{:?}", utc);
    let utc_string = utc.to_string();
    println!("{}", utc_string);
    let f = utc_string.parse::<DateTime<Utc>>().unwrap();
    println!("time : {:?}", f);
}
