#![allow(dead_code)]
///reference
use std::fmt::Display;
/// 右上 - 左下对角线上的两元素交换
/// ( 1.1 1.2 )
/// ( 2.1 2.2 )
/// 目前想到的一种方法
fn transpose(pair: Matrix) -> Matrix {
    let a = pair.0;
    let b = pair.1;
    let c = pair.2;
    let d = pair.3;
    Matrix(a, c, b, d)
}

// 在 “动手试一试” 的练习中要用到下面这个结构体。
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = self.0;
        let b = self.1;
        let c = self.2;
        let d = self.3;
        write!(f, "({},{})", a, b)?;
        write!(f, "\n",)?;
        write!(f, "({},{})", c, d)
    }
}

#[test]
fn test_tuple() {
    // 包含各种不同类型的元组
    let tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    //使用.index访问
    println!("{},{}", tuple.0, tuple.11);
    // 元组嵌套
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("{},{}", tuple_of_tuples.0 .0, tuple_of_tuples.1 .1);

    // 但很长的元组无法打印  marco递归次数的限制【猜测】
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。

    println!("Matrix:\n{}", Matrix(1.1, 1.2, 2.1, 2.2));
    println!("Transpose:\n{}", transpose(Matrix(1.1, 1.2, 2.1, 2.2)))
}
