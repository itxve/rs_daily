#![allow(dead_code)]

#[test]
fn test_match() {
    let number = 5;

    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值 单个依旧能够使用区间
        2..=5 | 7 | 11 => println!("This is a prime"),
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        // 处理其他情况
        _ => println!("Orther"),
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
        // 试一试 ^ 将其中一条分支注释掉
    };

    println!("{} -> {}", boolean, binary);
}
