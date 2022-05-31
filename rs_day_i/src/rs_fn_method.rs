#![allow(dead_code)]

// 二元操作std::ops
use std::ops::Mul;

struct Point {
    x: f32,
    y: f32,
}

// 实现的代码块，`Point` 的所有方法都在这里给出
impl Point {
    // 这是一个静态方法（static method）
    // 静态方法不需要被实例调用
    // 这类方法一般用作构造器（constructor）
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 另外一个静态方法，需要两个参数：
    fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
    // 这是一个实例方法（instance method）
    // `&self` 是 `self: &Self` 的语法糖（sugar），其中 `Self` 是方法调用者的
    // 类型。在这个例子中 `Self` = `Point`
    fn area(&self) -> f32 {
        self.x * self.y
    }

    //self会触发所有权的move
    fn scale_x(&mut self, scale: u8) {
        //同类型的才能二元操作，否则需要实现 Mul ，f32*u8 [???]不知道如何实现有知道的大佬告知下
        self.x = self.x * (scale) as f32;
    }
}

//实现自定义类型的二元操作
impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[test]
fn test_method() {
    let mut point = Point { x: 10.0, y: 8.0 };
    println!("{}", point.area());

    point.scale_x(2);

    println!("{}", point.area());

    let point = Point { x: 8.0, y: 2.0 };
    let point2 = Point { x: 2.0, y: 8.0 };

    let mul = point * point2;

    println!("{},{}", mul.x, mul.y);
}
