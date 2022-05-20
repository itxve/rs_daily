#![allow(dead_code)]

// `NanoSecond` 是 `u64` 的新名字。
type NanoSecond = u64;
type Inch = u64;

type Alias = NkAlias;

#[derive(Debug)]
struct NkAlias {
    x: u8,
}

// 通过这个属性屏蔽警告。
#[allow(non_camel_case_types)]
type u64_t = u64;
#[test]
fn test_alias() {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // 注意类型别名*并不能*提供额外的类型安全，因为别名*并不是*新的类型。
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );

    println!("{:?}", Alias { x: 10 })
}
