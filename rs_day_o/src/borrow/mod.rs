mod _ref;
mod alias;
mod immutable;

// 此函数取得一个 box 的所有权并销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// 此函数借用了一个 i32 类型
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

#[test]
fn main() {
    let box123 = Box::new(123);
    borrow_i32(&box123);
    borrow_i32(&box123);
    // eat_box_i32(box123);
    // 无法借用一个move的变量
    // borrow_i32(&box123);
    {
        // E0505 解释的还算清楚
        // 取得一个对 box 中数据的引用
        let _ref_to_i32 = &box123;

        // 报错！
        // 当 `boxed_i32` 里面的值之后在作用域中被借用时，不能将其销毁。
        // eat_box_i32(box123);
        // 改正 ^ 注释掉此行

        // 在 `_ref_to_i32` 里面的值被销毁后，尝试借用 `_ref_to_i32`
        //（译注：如果此处不借用，则在上一行的代码中，eat_box_i32(boxed_i32)可以将 `boxed_i32` 销毁。）
        borrow_i32(&_ref_to_i32);
        // `_ref_to_i32` 离开作用域且不再被借用。
    }
}
