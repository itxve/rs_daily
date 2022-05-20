#![allow(dead_code)]

/// https://rustwiki.org/zh-CN/rust-by-example/custom_types/constants.html
#[derive(Debug)]
struct SomeStruct {
    x: usize,
}

const FOO: SomeStruct = SomeStruct { x: 1 };
const BAR: &'static SomeStruct = &FOO;
//
static mut TOO: SomeStruct = FOO;

fn x_value() -> usize {
    // 在一般函数中访问常量
    unsafe { TOO.x }
}

#[test]
fn test_const() {
    use std::ptr;

    println!("{:?}", FOO);

    println!("{:?}", BAR);
    //修改可变的全局变量需要在unsafe块操作，编译期会告诉你😄
    unsafe {
        TOO.x = 1000;
        println!("{:?}", TOO);
    }
    println!("{}", x_value());

    unsafe { println!("FOO ref eq TOO [{}]", ptr::eq(&FOO, &TOO)) }

    println!("FOO ref eq BAR  [{}]", ptr::eq(&FOO, BAR))
}
