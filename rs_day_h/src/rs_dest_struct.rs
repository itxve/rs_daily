#![allow(dead_code)]

struct Bo {
    p: (u8, u8),
    t: String,
}
#[test]
fn test_dest_struct() {
    let bo = Bo {
        p: (5, 10),
        t: String::from("tr"),
    };
    let Bo { p: (p1, p2), t } = bo;

    println!("p1:{},p2:{},t:{}", p1, p2, t);

    // 解构会涉及所有权的移动 t 无法再使用
    let Bo { p: (p1, p2), .. } = bo;
    println!("{},{}", p1, p2)
}
