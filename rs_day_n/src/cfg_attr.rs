#![allow(dead_code)]

#[cfg_attr(a, test)]
// predicate, 属性列表
// #[cfg_attr(predicate, attr)]
fn main() {
    println!("{}  ", cfg!(a = "123"))
}
