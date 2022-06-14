#![allow(dead_code)]

use std::marker::PhantomData;

// 这个虚元组结构体对 `A` 是泛型的，并且带有隐藏参数 `B`。
// 注意：对于泛型 `A` 会分配存储空间，但 `B` 不会。
// 因此，`B` 不能参与运算。
#[derive(PartialEq)] // 允许这种类型进行相等测试（equality test）。
struct PhantomTuple<A, B>(A, PhantomData<B>);

// 这个虚类型结构体对 `A` 是泛型的，并且带有隐藏参数 `B`。
#[derive(PartialEq)] // 允许这种类型进行相等测试。
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

struct Klo();
// 默认泛型站位
pub trait AddC<RHS = bool> {
    type Output;
    fn add(&self, rhs: RHS) -> Self::Output;
}

impl AddC<u8> for Klo {
    type Output = u8;

    fn add(&self, rhs: u8) -> Self::Output {
        println!("u8: {}", rhs);
        rhs
    }
}

impl AddC<i32> for Klo {
    type Output = i32;

    fn add(&self, rhs: i32) -> Self::Output {
        println!("i32: {}", rhs);
        rhs
    }
}

#[test]
fn main() {
    // 这里的 `f32` 和 `f64` 是隐藏参数。
    // 被指定为 `<char, f32>` 的 `PhantomTuple` 类型。
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // 被指定为 `<char, f64>` `PhantomTuple` 类型。
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // 被指定为 `<char, f32>` 的类型。
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // 被指定为 `<char, f64>` 的类型。
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let kl = Klo();
    kl.add(-88);
    // 强制指定类型
    kl.add(133u8);
}
