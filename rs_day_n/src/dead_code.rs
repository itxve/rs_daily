// #![allow(dead_code)] // 文件顶部声明该模块禁用 `dead_code` lint

fn used_function() {
    println!("used_function")
}

// 单方法禁用 `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

#[test]
fn main() {
    used_function();
}
