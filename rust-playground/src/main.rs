#![deny(clippy::all)]
use std::convert::TryInto;
fn main() {
    //? i8 to i128
    let i_8: i8 = 5;
    let i_16: i16 = i16::from(i_8);
    let i_32: i32 = i32::from(i_16);
    let i_64: i64 = i64::from(i_32);
    let i_128: i128 = i128::from(i_64);
    println!("i 128 : {}", i_128);

    //? i128 to i8
    let i_8: Result<i8, _> = i_128.try_into();
    match i_8 {
        Ok(num_i8) => println!("Converted to i8 {}", num_i8),
        Err(_) => println!("Error : The value is outside of i8"),
    }

    //? u8 to u128
    let u_8: u8 = 5;
    let u_16: u16 = u16::from(u_8);
    let u_32: u32 = u32::from(u_16);
    let u_64: u64 = u_32 as u64; // as also support
    let u_128: u128 = u128::from(u_64);
    println!("u 128 : {}", u_128);

    //? u128 to u8
    let u_8: Result<u8, _> = u_128.try_into();
    match u_8 {
        Ok(num_u8) => println!("u128 Converte to u8 {}", num_u8),
        Err(_) => println!("Error : The value is outside of u8"),
    }

    //? i16 to u16
    let u_16: Result<u16, _> = i_16.try_into();
    match u_16 {
        Ok(num_u16) => println!("ui16 Converte to u16 {}", num_u16),
        Err(_) => println!("Error : The value is outside of u16"),
    }
    //? u32 to i32
    let i_32: Result<i32, _> = u_32.try_into();
    match i_32 {
        Ok(num_i32) => println!("u32 Converte to u8 {}", num_i32),
        Err(_) => println!("Error : The value is outside of i32"),
    }
}
