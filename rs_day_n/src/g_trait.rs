#![allow(dead_code)]

#[derive(Debug)]
struct S; // 具体类型 `S`

#[derive(Debug)]
struct GenericVal<T>(T); // 泛型类型 `GenericVal`

// 一个待实现的特征 ，类似其他语言的接口
pub trait Print<T> {
    fn pt(&self, _t: &T) {}
}

// GenericVal 的 `impl`，此处我们显式地指定了类型参数：
// 指定 `f32` 类型
impl Print<f32> for GenericVal<f32> {
    fn pt(&self, t: &f32) {
        println!("f32 :{:?}, t:{}", self, t);
    }
}
impl Print<S> for GenericVal<S> {
    fn pt(&self, t: &S) {
        println!("f32 :{:?}, t:{:?}", self, t);
    }
} // 指定为上面定义的 `S`

#[test]
fn generic_tarit() {
    let fg = GenericVal(33.0);
    fg.pt(&fg.0);
    let sg = GenericVal(S);
    sg.pt(&sg.0);
}
