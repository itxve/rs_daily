#![allow(dead_code)]

struct Vm(i32);

trait Tom {}

impl Tom for Vm {}

impl Debug for Vm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Debug For Vm").field(&self.0).finish()
    }
}

use std::fmt::Debug;

// T 同时实现了Tom 和 Debug
fn print_debug<T>(t: &T)
where
    T: Tom + Debug,
{
    println!("{:?}", t);
}

#[test]
fn main() {
    print_debug(&Vm(143));
}
