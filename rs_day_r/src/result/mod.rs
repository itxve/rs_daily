#![allow(dead_code)]
// 一个用户数字转换的简单宏
// #[macro_export]
macro_rules! parse_number {
    ($string:expr,$type:ident) => {
        // 为了有返回类型, 宏的block貌似也是返回一个 Result
        {
            let rt = $string.parse::<$type>().unwrap();
            rt
        }
    };
}
// // 当前crate
// pub(crate) use parse_number;

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // 我们试着用 `unwrap()` 把数字放出来。它会咬我们一口吗？
    let first_number = parse_number!(first_number_str, i32);
    let second_number = parse_number!(second_number_str, i32);
    first_number * second_number
}

#[test]
fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}

mod the_map;

mod alias;

mod early_returns;
mod enter_question_mark;
