#![allow(dead_code)]

#[test]
fn test_if_else() {
    let pc = 100;
    if pc < 100 {
        println!("pc is small than 100");
    } else if pc > 100 {
        println!("pc is large than 100");
    } else {
        println!("pc is  100");
    }

    let big_n = if pc < 10 && pc > -10 {
        println!(", and is a small number, increase ten-fold");
        // 这个表达式返回一个 `i32` 类型。
        10 * pc
    } else {
        println!(", and is a big number, half the number");
        // 这个表达式也必须返回一个 `i32` 类型。
        pc / 2
        // 试一试 ^ 试着加上一个分号来结束这条表达式。
    };
    //   ^ 不要忘记在这里加上一个分号！所有的 `let` 绑定都需要它。

    println!("{} -> {}", pc, big_n);

    //一个复杂的if   if块 ({ any code })
    let nest_if = if ({
        if pc % 3 == 0 {
            12
        } else {
            0
        }
    }) > 0
    {
        123
    } else {
        0
    };

    println!("nest_if is {}", nest_if)
}
