use std::error::Error;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
// 定义我们的错误类型，这种类型可以根据错误处理的实际情况定制。
// 我们可以完全自定义错误类型，也可以在类型中完全采用底层的错误实现，
// 也可以介于二者之间。
struct DoubleError {
    desc: String,
}

impl DoubleError {
    fn of(err: String) -> Self {
        Self { desc: err }
    }
}

// 错误的生成与它如何显示是完全没关系的。没有必要担心复杂的逻辑会导致混乱的显示。
//
// 注意我们没有储存关于错误的任何额外信息，也就是说，如果不修改我们的错误类型定义的话，
// 就无法指明是哪个字符串解析失败了。
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double the desc is {}", self.desc)
    }
}

// 为 `DoubleError` 实现 `Error` trait，这样其他错误可以包裹这个错误类型。
impl Error for DoubleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // 把错误换成我们的新类型。
        .ok_or(DoubleError::of(String::from("第一个数字")))
        .and_then(|s| {
            s.parse::<i32>()
                // 这里也换成新类型。
                .map_err(|_| DoubleError::of(String::from("第二个数字")))
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
