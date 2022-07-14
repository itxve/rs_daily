use std::ops::{self, Add};

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// `std::ops::Add` trait 用来指明 `+` 的功能，这里我们实现 `Add<Bar>`，它是用于
// 把对象和 `Bar` 类型的右操作数（RHS）加起来的 `trait`。
// 下面的代码块实现了 `Foo + Bar = FooBar` 这样的运算。
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// 通过颠倒类型，我们实现了不服从交换律的加法。
// 这里我们实现 `Add<Foo>`，它是用于把对象和 `Foo` 类型的右操作数加起来的 trait。
// 下面的代码块实现了 `Bar + Foo = BarFoo` 这样的运算。
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

#[test]
fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

struct FooTuple(u16, u16);
struct BarTuple(u16, u16);
#[derive(Debug)]
struct ResultTuple(u32, u32);
// FooTuple + BarTuple 的实现
impl Add<BarTuple> for FooTuple {
    type Output = ResultTuple;
    fn add(self, rhs: BarTuple) -> Self::Output {
        ResultTuple((self.0 + rhs.0).into(), (self.1 + rhs.1).into())
    }
}

#[test]
fn foo_bar_tuple() {
    let ft = FooTuple(10, 5);
    let bt = BarTuple(5, 10);
    println!("FooTuple + BarTuple = {:?}", ft + bt);
}