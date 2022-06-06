// 此声明将会查找名为 `orther.rs` 或 `orther/mod.rs` 的文件，并将该文件的内容放到
// 此作用域中一个名为 `orther` 的模块里面。
mod orther;

mod netest;

#[test]
fn main() {
    orther::info();

    // 使用重新导出
    netest::nc::nc_console();
}
