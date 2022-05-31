#![allow(dead_code)]

use core::time;
use std::mem;

/**
 * 闭包默认捕获的是变量引用
 */
#[test]
fn main() {
    test_capture();
    test_capture_mut_var();
    test_non_copyt();
    test_move();
    test_fn_var();
}

///  `FnOnce` 换成 `Fn` 或 `FnMut`。控制闭包改如何使用
#[test]
fn test_fn_var() {
    // 该函数将闭包作为参数并调用它。
    fn apply<F>(f: F)
    where
        // 闭包没有输入值和返回值。
        F: FnOnce(),
    {
        // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。
        f();
        // FnOnce 表示捕获方式为通过值（T）的闭包，无法再次使用
        // f();
        println!("apply called...")
    }

    let close_fn = || println!("close_fn");
    apply(close_fn);
    apply(close_fn);

    // 输入闭包，返回一个 `i32` 整型的函数。
    fn apply_to_3<F>(f: F) -> i32
    where
        // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    let rt_apply = apply_to_3(|x| {
        assert_eq!(x, 3, "x not is 3 ");
        x * 2
    });
    println!("rt_apply :{}", rt_apply);

    //函数指针
    fn fn_ptr(x: i32) -> i32 {
        x * 20
    }
    let usr_fn_ptr = apply_to_3(fn_ptr);
    println!("usr_fn_ptr :[{}] ,value is 60", usr_fn_ptr);
}

#[test]
fn test_fn_gj() {
    fn apply_fn_mut_by_times<F>(mut f: F, times: u8)
    where
        // 一个没有输入值和返回值闭包。
        F: FnMut(),
    {
        //调用次数
        for _ in 1..=times {
            f();
        }
    }

    let mut c = 0;
    apply_fn_mut_by_times(
        || {
            c += 1;
        },
        8,
    );
    println!("c now is {}", c);

    // 实现一个从零开始，讲闭包返回值累加方法
    fn apply_fn_mut_by_times_sum<F>(f: F, times: u8) -> u32
    where
        F: Fn(u8) -> u32,
    {
        let mut x = 0;
        for i in 0..times {
            x = x + f(i);
            println!("x: {}", x)
        }
        x
    }

    let sum_c = apply_fn_mut_by_times_sum(|x| (x + 1) as u32, 100);
    println!("sum_c now is {}", sum_c);

    let sum_c = apply_fn_mut_by_times_sum(|x| (x + 2) as u32, 75);
    println!("sum_c now is {}", sum_c);
}

#[test]
fn test_move() {
    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3];

    //强制移动所有权   //闭包
    let contains = move |needle| {
        // hello被move,导致 contains无法二次调用
        haystack.contains(needle)
    };

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。

    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。
}

#[test]
fn test_non_copyt() {
    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // [可复制类型]将会复制给闭包，从而原始值不受影响。
    // 不可复制类型必须移动（move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        // 释放一个变量
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
}

#[test]
fn test_capture_mut_var() {
    let mut count = 0;
    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    inc();

    let _count_reborrowed = &mut count;
    //解引用
    *_count_reborrowed += 12;

    println!("_count_reborrowed: {}", _count_reborrowed)
}

#[test]
fn test_capture() {
    let color = String::from("green");

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。

    let print = || println!("`color`: {}", color);
    print();

    //但是闭包本身存储到 `print` 变量中。`color` 会一直保持状态直到`print` 离开作用域。
    // 这里的作用域我的理解是 test_capture方法梯中
    let print = move || println!("`color`: {}", color);
    print();
    print();
    // color的所有权被移动 ,无法使用
    // println!("`color`: {}", color);
}
