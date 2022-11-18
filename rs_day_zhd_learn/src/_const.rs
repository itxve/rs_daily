#![allow(dead_code)]

use core::mem::MaybeUninit;

const fn sd() -> i32 {
    10
}

const AD: i32 = sd();

#[derive(Debug)]
pub struct ArrayVec<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    length: usize,
}

impl<T: std::marker::Copy, const N: usize> ArrayVec<T, { N }> {
    #[inline]
    pub const fn new(len: usize) -> ArrayVec<T, { N }> {
        ArrayVec {
            items: [MaybeUninit::uninit(); N],
            length: len,
        }
    }
}

impl<T, const N: usize> Drop for ArrayVec<T, { N }> {
    fn drop(&mut self) {}
}

const fn e2(input: u8) -> u8 {
    if input > 10 {
        return 255;
    }
    0
}

#[derive(Clone, Copy, Debug)]
struct A(i8);

#[derive(Clone, Copy, Debug)]
struct AB(A);

#[repr(C)]
union MyUnion {
    f1: A,
    f2: AB,
    f3: u16,
}

impl std::fmt::Debug for MyUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?} , {:?} , {:?}",
            unsafe { self.f1 },
            unsafe { self.f2 },
            unsafe { self.f3 },
        ))
    }
}

#[test]
fn do_test() {
    let aw = MyUnion { f3: 88 };
    println!("as: {:?}", aw);

    let fg = e2(102);
    println!("{}", fg);

    let c: ArrayVec<i32, 2> = ArrayVec::new(4);
    println!("c :{:?}", c)
}
