#![allow(dead_code)]

#[test]
fn test_expression() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        // 将此表达式赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号结束了这个表达式，于是将 `()` 赋给 `z`
        2 * x;
        //
        // 10
    };
    let t = { 2 * x };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
    println!("t is {:?}", t);
}
