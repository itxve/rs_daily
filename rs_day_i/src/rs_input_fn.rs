#![allow(dead_code)]

// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_me<F: Fn()>(f: F) {
    f()
}

// 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
fn function() {
    println!("I'm a function!");
}

#[test]
fn main() {
    let x = 10;
    // 定义一个满足 `Fn` 约束的闭包。
    let closure = || println!("I'm a closure! capture a x :{}", x);

    call_me(closure);
    //外部函数无法捕获作用域变量
    call_me(function);
}
