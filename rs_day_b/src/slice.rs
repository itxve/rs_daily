#![allow(dead_code)]
use std::mem;

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
#[test]
fn test_slice() {
    analyze_slice(&[100, 23, 88, 72]);
    // 定长数组（类型标记是多余的）
    let xs: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 5] = [10; 5];
    println!("y={:?}", &ys);
    // 数组是在栈中分配的内存大小
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    //输出0,1下标对应的值 [0..=1]表示[0,1]闭区间
    println!("0-2 ele{:?} ", &xs[0..=1]);
}
