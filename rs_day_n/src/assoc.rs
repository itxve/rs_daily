#![allow(dead_code)]

// `A` 和 `B` 在 trait 里面通过 `type` 关键字来定义。
// （注意：此处的 `type` 不同于为类型取别名时的 `type`）。
trait Contains {
    type A;
    type B;

    // 这种语法能够泛型地表示这些新类型。
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
}

#[derive(Debug)]
struct Container(i32, i32);

// 使用关联类型
fn difference<C: Contains>(container: &C) -> &C {
    container
}

#[test]
fn mian() {
    let con = Container(10, 34);
    println!("contains is :{}", con.contains(&22, &45));

    println!("difference is :{:#?}", difference(&con));
}
