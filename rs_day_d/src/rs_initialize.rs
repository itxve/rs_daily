#![allow(dead_code)]
///reference https://rustwiki.org/zh-CN/rust-by-example/variable_bindings/declare.html
#[test]
fn main() {
    // 声明一个变量绑定
    let mut a_binding;
    {
        let x = 2;
        // 初始化一个绑定
        a_binding = x * x;
    }
    //不允许二次绑定 使用mut
    a_binding = 10;
    println!("a binding: {}", a_binding);

    let another_binding;

    // 报错！使用了未初始化的绑定
    // println!("another binding: {}", another_binding);
    // 改正 ^ 注释掉此行

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
