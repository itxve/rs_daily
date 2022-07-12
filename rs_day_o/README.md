# [作用域规则](https://rustwiki.org/zh-CN/rust-by-example/scope.html)

|        仓库文件索引          |         原文地址         |   
|----------------------|:-----------------------:| 
| [RAII(资源获取即初始化)](./src/raii.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/raii.html)
| [所有权和移动](./src/_move/) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/move.html)
| [借用](./src/borrow/) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/borrow.html)
| [可变性](./src/borrow/immutable.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/borrow/mut.html)
| [别名使用](./src/borrow/alias.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/borrow/alias.html)
| [ref 模式](./src/borrow/_ref.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/borrow/ref.html)
| [生命周期](./src/lifeme/) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime.html)
| [显式标注](./src/lifeme/explicit.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/explicit.html)
| [函数](./src/lifeme/_fn.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/fn.html)
| [方法](./src/lifeme/method.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/methods.html)
| [结构体](./src/lifeme/_struct.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/struct.html)
| [trait](./src/lifeme/_trait.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/trait.html)
| [约束](./src/lifeme/bounds.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/lifetime_bounds.html)
| [强制转换](./src/lifeme/coercion.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/lifetime_coercion.html)
| [static](./src/lifeme/_static.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/static_lifetime.html)
| [省略](./src/lifeme/elision.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/elision.html)





## 章节小总结

### 借用规则
- 数据可以多次不可变借用，但是在不可变借用的同时，原始数据不能使用可变借用。
- 同一时间内只允许一次可变借用。
- 仅当最后一次使用可变引用之后，原始数据才可以再次借用。

### 生命周期
- 生命周期一般和引用挂钩
- 短生命周期不能强制转换成长生命周期。


## 案例写法
- 单个泛型
 ```
//这里接受一个指向 `T` 的引用，其中 `T` 实现了 `Debug` trait，并且在 `T` 中的
//所有*引用*都必须比 `'a'` 存活时间更长。另外，`'a` 也要比函数活得更长。
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}
```

- 自动推导
 ```
// `<'a: 'b, 'b>` 读作生命周期 `'a` 至少和 `'b` 一样长。
// 在这里我们我们接受了一个 `&'a i32` 类型并返回一个 `&'b i32` 类型，这是
// 强制转换得到的结果。
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}
```

