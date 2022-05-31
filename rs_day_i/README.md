# [函数](https://rustwiki.org/zh-CN/rust-by-example/fn.html)

|        仓库文件索引          |         原文地址         |   
|----------------------|:-----------------------:|
| [方法](./src/rs_fn_method.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/fn/methods.html)| 
| [闭包](./src/rs_fn_closure.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/fn/closures.html)| 
| [捕获](./src/rs_fn_capture.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/fn/closures/capture.html)| 
| [输入函数](./src/rs_input_fn.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/fn/closures/input_parameters.html)| 
| [输出参数](./src/rs_output_fn.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/fn/closures/output_parameters.html)| 
| [标准库中的例子](./src/rs_iterator_ex.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/fn/closures/closure_examples/iter_any.html)| 
| [高阶函数](./src/rs_iterator_ex.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/fn/hof.html)|





## 章节小总结

 ```
 闭包本质上很灵活，能做功能要求的事情，使闭包在没有类型标注的情况下运行。这使得捕获（capture）能够灵活地适应用例，既可移动（move），又可借用（borrow）。闭包可以通过以下方式捕获变量：

- 通过引用：&T
- 通过可变引用：&mut T
- 通过值：T
闭包优先通过引用来捕获变量，并且仅在需要时使用其他方式。
 ```
