#![allow(dead_code)]

// `age` 函数，返回一个 `u32` 值。
fn age() -> u32 {
    15
}
#[test]

fn test_match_bind() {
    match age() {
        //绑定匹配到的值很有用
        new_n @ 5..=20 => {
            println!("5..=20 :{} ", new_n);
        }
        _n => {
            println!("new_n :{} ", _n);
        }
    }
    let gt = [1, 3, 5, 7, 9];
    // 绑定元组
    let [c @ .., l, h] = gt;
    println!("c: {:?} ,l: {:?} h: {}", c, l, h);

    let op = Some(-199);
    match op {
        Some(n @ i32::MIN..=0) => {
            println!("u32::MIN..-0 :{}", n);
        }
        Some(n @ 1..=3) => {
            println!("1..=3 :{}", n);
        }
        Some(n @ 4..) => {
            println!("4.. :{}", n);
        }
        None => {}
    }
}
