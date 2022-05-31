#![allow(dead_code)]

#[test]
fn main() {
    let sum_of_squared_odd_numbers: u32 = (0..=5)
        .map(|n| n * n) // 所有自然数取平方
        .take_while(|&n| n < 100) // 取小于上限的
        .filter(|&n| n % 2 == 1) // 取奇数
        .fold(0, |sum, i| sum + i); // 最后加起来
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
