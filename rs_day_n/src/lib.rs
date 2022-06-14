#![allow(dead_code)]

use std::fmt::Display;

mod assoc;
mod g_bounds;
mod g_fn;
mod g_impl;
mod g_trait;
mod new_type;
mod phantom;

#[derive(Debug)]
struct Aa<T>(T);

impl<T: Display> Aa<T> {
    fn print(&self) {
        match self {
            Aa(t) => {
                println!("T value is : [{}]", t)
            }
        }
    }
}

fn generic(_s: Aa<&str>) {
    _s.print();
}

#[test]
fn main_() {
    Aa(123).print();
    generic(Aa("#@123@#"));
}
