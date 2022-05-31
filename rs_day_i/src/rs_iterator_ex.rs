#![allow(dead_code)]

type Vex = [i32; 4];

#[test]
fn main() {
    let vec1 = vec![1, 2, 3];
    let mut vec2 = vec![2, 5, 6, 8];

    // 对 vec 的 `iter()` 举出 `&i32`。（通过用 `&x` 匹配）把它解构成 `i32`。
    // 译注：注意 `any` 方法会自动地把 `vec.iter()` 举出的迭代器的元素一个个地
    // 传给闭包。因此闭包接收到的参数是 `&i32` 类型的。
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 举出 `i32` 类型。无需解构。

    // 可变
    vec2.iter_mut().for_each(|x| {
        if x == &2 {
            *x = 18;
        }
    });
    println!("2 in array1: {:?}", vec2);

    let array1 = [1, 2, 3];
    let array2: Vex = [4, 5, 6, 7];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));

    println!("find x in array2: {:?}", array2.iter().find(|&&x| x == 5));
}
