#![allow(dead_code)]
#[test]
fn test_mut() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // 正确代码 Rust 并没有++操作
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 错误！
    // _immutable_binding += 1;
    // 改正 ^ 将此行注释掉
}
