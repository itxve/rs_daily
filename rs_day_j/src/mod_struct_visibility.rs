#![allow(dead_code)]

mod st {
    // 一个公有的结构体，带有一个公有的字段（类型为泛型 `T`）
    #[derive(Debug)]
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 一个公有的结构体，带有一个私有的字段（类型为泛型 `T`）
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 一个公有的构造器方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

#[test]
fn main() {
    let ob = st::OpenBox { contents: "123" };
    println!("ob's contents : {}", ob.contents);

    let cb = st::ClosedBox::new("Close Box");
    println!("cb: {:?}", cb);
}
