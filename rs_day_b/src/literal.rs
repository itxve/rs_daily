/// reference https://rustwiki.org/zh-CN/rust-by-example/primitives/literals.html
#[test]
fn prnt() {
    // 整数相加
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);
    // 试一试 ^ 尝试将 `1i32` 改为 `1u32`，编译不通过
    // println!("1 - 2 = {}", 1u32 - 2);
    // 短路求值的布尔逻辑
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 二进制的位运算
    //与运算 两个位都为1时，结果才为1
    println!("0011 [与运算｜AND] 0101 is {:04b}", 0b0011u32 & 0b0101);
    //或运算	两个位都为0时，结果才为0
    println!("0011 [或运算｜OR] 0101 is {:04b}", 0b0011u32 | 0b0101);
    //异或运算	两个位相同为0，相异为1
    println!("0011 [异或运算｜XOR] 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    //左移5位
    println!("1 << 5 is {}", 1u32 << 5);
    //右移2位
    println!("0x80 >> 2 is 0x{:x},{}", 0x80u32 >> 2, 0x80u32);
    // 使用下划线改善数字的可读性！
    println!("One million is written as {}", 1_000_000u32);
}
