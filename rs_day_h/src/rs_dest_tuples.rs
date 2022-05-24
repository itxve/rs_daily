#![allow(dead_code)]

#[test]
fn test_dest_tuples() {
    let triple = (1, -2, 3);
    let (x, ..) = triple;
    println!("x is:{}", x);

    match triple {
        // match匹配到了就会中断，不再继续匹配
        (1, -2, ..) => println!("First is `1` and the rest doesn't matter"),
        // 解构出第二个和第三个元素
        (1, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        // `..` 可用来忽略元组的其余部分
        _ => println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
}
