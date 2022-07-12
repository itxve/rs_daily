mod immutable;
mod partial_move;

// 此函数取得堆分配的内存的所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` 被销毁且内存得到释放
}

#[test]
fn main() {
    let bo = Box::new(100);
    destroy_box(bo);
    // 已经被move 无法继续使用
    // bo;

    // 栈分配的整型
    let x = 5u32;
    // 将 `x` *复制*到 `y`——不存在资源移动
    let y = x;

    // 两个值各自都可以使用
    println!("x is {}, and y is {}", x, y);

    // `a` 是一个指向堆分配的整数的指针
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *移动* `a` 到 `b`
    let b = a;
    //moved
    // println!("a contains: {}", a);
    println!("b contains: {}", b);
}
