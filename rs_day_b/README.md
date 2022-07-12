
## [原生类型](https://rustwiki.org/zh-CN/rust-by-example/primitives.html)

|        仓库文件索引          |         原文地址         |        
|----------------------|:-----------------------:|
| [字面量和运算符](./src/literal.rs) | [原文](https://rustwiki.org/zh-CN/rust-by-example/primitives/literals.html)     
| [元组](./src/tupe.rs)  |  [原文](https://rustwiki.org/zh-CN/rust-by-example/primitives/tuples.html)
| [数组和切片](./src/slice.rs)   |  [原文](https://rustwiki.org/zh-CN/rust-by-example/primitives/array.html)


### 章节小总
### 长度超过12的元组无法打印  marco递归次数的限制【猜测】
- [原因](https://github.com/rust-lang/rust/blob/master/library/core/src/fmt/mod.rs#L2303-L2332)