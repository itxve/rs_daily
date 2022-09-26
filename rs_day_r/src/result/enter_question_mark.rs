use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = try_pass(first_number_str)?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn try_pass(first_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>()
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn main() {
    print(multiply("10", "2"));
    print(multiply("5", "2"));
}
