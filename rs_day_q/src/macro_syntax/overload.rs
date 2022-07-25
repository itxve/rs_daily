macro_rules! test {
    // 参数不需要使用逗号隔开。
    // 参数可以任意组合！
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // ^ 每个分支都必须以分号结束。
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
    ($left:expr; = $right:expr) => {
        println!(
            "{:?} == {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left == $right
        )
    };
    ($left:expr; > $right:expr) => {
        println!(
            "{:?} > {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left > $right
        )
    };
    ($left:ident, $right:expr) => {
        println!(
            "{:?} > {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left > $right
        )
    };
}

#[test]
fn main() {
    test!({1i32 + 1 == 2i32}; and 2i32 * 2 == 4i32);
    test!({true}; or {false});
    test!(3;=3);
    test!(3;>3);
}
