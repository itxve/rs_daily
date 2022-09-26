#![allow(dead_code)]

use std::num::ParseIntError;

mod option_unwrap;
#[macro_use]
mod result;
mod iter_result;
mod multiple_error_types;

// 修改了上一节中的返回类型，现在使用模式匹配而不是 `unwrap()`。
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str
        .parse::<i32>()
        .and_then(|fns| second_number_str.parse::<i32>().map(|sns| fns * sns))
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
#[test]
fn main2() {
    let vd = multiply("3e", "ty");
    println!("{:?}", vd)
}
