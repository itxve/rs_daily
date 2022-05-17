/// reference https://rustwiki.org/zh-CN/rust-by-example/primitives.html
mod literal;
mod slice;
mod tupe;

#[cfg(test)]
mod tests {

    #[test]
    fn p8() {
        // 有符号整数（signed integers）：i8、i16、i32、i64、i128 和 isize（指针宽度）
        let i8_l: i8 = -128;
        let i8_r: i8 = 127;
        println!("{},{}", i8_l, i8_r);
        // 无符号整数（unsigned integers）： u8、u16、u32、u64、u128 和 usize（指针宽度）
        let u8_l: u8 = 0;
        let u8_r: u8 = 127;
        println!("{},{}\nU,{},{}", i8_l, i8_r, u8_l, u8_r,)
    }

    // 单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）
    #[test]
    fn pchar() {
        let c: char = '😄';
        println!("{}", c)
    }

    //数字还可以通过后缀（suffix）或默认方式来声明类型
    #[test]
    fn pf32() {
        let a: f32 = 12.5;
        let b = 12f32;
        println!("{},{}", a, b)
    }
}
