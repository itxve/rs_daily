#![allow(dead_code)]

use std::{error::Error, num::ParseIntError, string::ParseError};

struct Po {
    x: i32,
    y: i32,
}
fn sum(a: i32, ref b: i32) -> i32 {
    a + b
}

#[test]
fn do_let() {
    let (a, b, c, _f) = (1, 2, 3, 4);
    println!("{},{},{},{}", a, b, c, _f);

    let Po { x: _x, y } = Po { x: (1), y: (2) };
    println!("{},{}", _x, y);

    let _sum = sum(10, 5);
    println!("{}", _sum);
}

#[test]
fn do_ref() {
    let a = 30;
    let b = &a;
    // 引用匹配
    let ref c = a;

    assert_eq!(b, c, "we are testing addition with {} and {}", b, c);

    let mut arr = [1, 3, 4];
    let ref mut brr = arr;
    brr[2] = 7;
    arr[0] = 9;
    println!("{:?}", arr)
}

#[test]
fn d_match() {
    /**
     *Option 表示是否有值
     */
    fn _option(input: Option<&str>) {
        match input {
            Some(ref v) => {
                println!("{}", v)
            }
            None => {
                println!("None ...");
            }
        }
    }
    _option(Some("120"));
    _option(None);

    fn parse_result(input: i32) -> Result<i32, &'static str> {
        if input > 10 {
            Ok(input)
        } else {
            Err("a error ")
        }
    }

    let cf = parse_result(3);
    println!("{:?}", cf);

    fn do_match_string(x: &Option<String>) {
        // match x {
        //     &Some(ref s) => {
        //         println!("{}", s)
        //     }
        //     &None => {
        //         println!("None ..c")
        //     }
        // }

        // match *x {
        //     Some(ref s) => {
        //         println!("{}", s)
        //     }
        //     None => {
        //         println!("None ..c")
        //     }
        // }
        // 编译器自动加ref
        match x {
            Some(s) => {
                println!("{}", s)
            }
            None => {
                println!("None ..c")
            }
        }
    }
    do_match_string(&Some(String::from("cvf")))
}

#[test]
fn do_match_arr() {
    let arr = [1, 2, 3, 4];
    match arr {
        [2, 3, 4, 4] => {
            println!("")
        }
        _ => {
            println!("arr default")
        }
    }
    match arr[..] {
        [1, 2, 3, 4] => {
            println!("all")
        }
        [1, 2, ..] => {
            println!("pre 2")
        }

        _ => {}
    }
}

#[test]
fn do_if_let() {
    // let vc = &Some(12);
    let vc: &Option<i32> = &None;
    //引用->非引用 编译器会自动加ref  ,可以使用代码块
    if let Some(x) = { vc } {
        println!("x:  {}", x)
    } else if let None = { vc } {
        println!("None x:")
    }

    let mut bc = 12;

    //引用->非引用 编译器会自动加ref
    if let (mut nx) = bc {
        nx = 44;
        println!("nx:  {}", nx);
    }

    println!("bc:  {:?}", bc)
}

#[test]
fn do_while_let() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

fn is_hello<T: AsRef<str>>(s: T) {
    println!("{}", s.as_ref())
}

#[test]
fn ksd() {
    let s = "hello";
    is_hello(s);

    let s = "hello".to_string();
    is_hello(s);
}
