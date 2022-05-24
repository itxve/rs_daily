#![allow(dead_code)]

#[test]
fn test_while() {
    let mut n = 1;
    // 当 `n` 小于 101 时循环
    while n < 52 {
        if n % 15 == 0 {
            println!("while fizzbuzz");
        } else if n % 3 == 0 {
            println!("while fizz");
        } else if n % 5 == 0 {
            println!("while buzz");
        } else {
            println!("while {}", n);
        }
        // 计数器值加 1
        n += 1;
    }
    println!("-------------------------------");
    let mut n = 1;
    loop {
        if n >= 52 {
            break;
        }
        if n % 15 == 0 {
            println!("loop fizzbuzz");
        } else if n % 3 == 0 {
            println!("loop fizz");
        } else if n % 5 == 0 {
            println!("loop buzz");
        } else {
            println!("loop {}", n);
        }
        // 计数器值加 1
        n += 1;
    }
}
