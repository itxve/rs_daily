#![allow(dead_code)]

#[derive(Debug)]
struct GenericVal<T>(T); // 泛型类型 `GenericVal`

// GenericVal 的 `impl`，此处我们显式地指定了类型参数：
// 指定 `f32` 类型
impl GenericVal<f32> {
    fn pt(&self) {
        println!("f32 :{:?}", self);
    }
}

#[test]
fn generic_impl() {
    let fg = GenericVal(33.0);
    fg.pt();
}
