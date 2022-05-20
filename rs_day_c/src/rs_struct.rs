#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单元结构体
struct Unit;

// 元组结构体
struct Pair(u8, u8, u8, u8);

// 带有两个字段的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    // 可以在空间中给定左上角和右下角在空间中的位置来指定矩形。
    top_left: Point,
    bottom_right: Point,
}
impl Rectangle {
    fn area(&self) {
        let Point { x, y } = self.top_left;
        println!("x*y={}", x * y);
        //解构重命名
        let Point { x: xx, y: yy } = self.bottom_right;
        println!("xx*yy={}", xx * yy);
    }
}

//增加一个函数 square，接受的参数是一个 Point 和一个 f32，并返回一个 Rectangle（长方形）
//，其左下角的点等于 Point 参数，长和宽都等于 f32 参数。
// tip:长和宽=w,
fn square(point: &Point, w: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y + w,
        },
        bottom_right: Point {
            x: point.x + w,
            y: point.y,
        },
    }
}

#[test]
fn test_struct() {
    let person = Person {
        age: 19,
        name: String::from("菜菜"),
    };
    println!("{:#?}", person);

    let point = Point { x: 1.0, y: 10.0 };
    println!("{:#?}", point);

    // 结构更新语法，补齐改结构体剩下的字段
    let new_point = Point { x: 1.0, ..point };
    println!("{:#?}", new_point);

    let pair = Pair(10, 4, 12, 23);
    //..	.., expr.., ..expr, expr..expr	右排除范围
    let Pair(first, .., last) = pair;

    println!("first ele:{},last ele:{}", first, last);

    let rect = Rectangle {
        top_left: point,
        bottom_right: new_point,
    };

    println!("{:#?}", rect);
}

#[test]
fn demo() {
    let point_tl = Point { x: 8.0, y: 10.0 };
    let point_br = Point { x: 8.0, y: 5.0 };
    let rect = Rectangle {
        top_left: point_tl,
        bottom_right: point_br,
    };
    rect.area();

    println!("square: {:?}", square(&Point { x: 0.0, y: 0.0 }, 1.0));
}
