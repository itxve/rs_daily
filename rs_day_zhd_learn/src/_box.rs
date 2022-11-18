use std::ops::Deref;
#[derive(Debug)]
struct Bn<T>(T);

impl<T> Deref for Bn<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        println!("11");
        &self.0
    }
}

struct User {
    n: &'static str,
}
impl User {
    fn name(&self) -> &'static str {
        self.n
    }
}

fn take_str(input: &str) {}

#[test]
fn do_box() {
    let vs = Bn(1);
    // * 会调用Deref::deref 进行解引用
    println!("{:?}", *vs);
    //.操作符自动解引用
    let uts = Bn(User { n: "123" });
    uts.name();
    //函数参数传递时自动解引用
    take_str(&String::from("abc"));
}
