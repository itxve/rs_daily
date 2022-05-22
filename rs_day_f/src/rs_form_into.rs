#![allow(dead_code)]

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// 将slice类型转换成Number类型
impl From<&str> for Number {
    fn from(item: &str) -> Self {
        let item = item.parse::<i32>().unwrap();
        Number { value: item }
    }
}
// ...orther impl

#[test]
fn test_form() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let str_form = Number::from("258");
    println!("My number is {:?}", str_form);
}

#[test]
fn test_into() {
    let int = 5;
    // 试试删除类型说明 ,指定type，编译器会自动推断其类型
    let num: Number = int.into();
    println!("My number is {:?}", num);

    let int = "887";
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
