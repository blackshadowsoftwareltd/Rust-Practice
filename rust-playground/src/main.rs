#![deny(clippy::all)]

fn main() {
    let x_str: &str = "10";
    let x_i32: i32 = x_str.parse::<i32>().unwrap();
    println!("x_str {} to x_i32 = {}", x_str, x_i32);

    let x_float: f32 = x_str.parse::<f32>().unwrap();
    println!("x_str {} to x_float = {}", x_str, x_float);

    let int_to_str: &str = &x_i32.to_string();
    println!("x_i32 {} to int_float_to_str = {}", x_i32, int_to_str);

    let float_to_str: &str = &x_float.to_string();
    println!("x_float {} to float_to_str = {}", x_float, float_to_str);

    let concat_string: String = format!(
        "{}, {}, {}, {}, {}",
        x_str, x_i32, x_float, int_to_str, float_to_str
    );
    println!("concat_String = {}", concat_string);

    let concat_str: &str = concat_string.as_str(); // String and str is not same type. So, need to convert String to str.
    println!("concat_str = {}", concat_str);
}
