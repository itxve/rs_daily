#![allow(dead_code)]

#[test]
fn main() {
    test_output();
}
//只有使用impl Trait才能返回一个闭包

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

#[test]
fn test_output() {
    let rt_fn = create_fn();
    rt_fn();
    rt_fn();

    let mut rt_mut = create_fnmut();
    rt_mut();
    rt_mut();
    rt_mut();

    let rt_once = create_fnonce();
    rt_once();
    // rt_once();
}
