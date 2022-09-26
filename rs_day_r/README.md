# [错误处理](https://rustwiki.org/zh-CN/rust-by-example/error.html)

|        仓库文件索引          |         原文地址         |   
|----------------------|:-----------------------:| 
| [Option 和 unwrap](./src/option_unwrap/option.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/option_unwrap.html)
| [组合算子：map](./src/option_unwrap/combine_map.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/option_unwrap/map.html)
| [组合算子：and_then](./src/option_unwrap/and_then.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/option_unwrap/and_then.html)
| [Result 的 map](./src/result/the_map.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/result/result_map.html)
| [给 Result 取别名](./src/result/alias.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/result/result_alias.html)
| [提前返回错误](./src/result/early_returns.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/result/early_returns.html)
| [引入 ?](./src/result/enter_question_mark.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/result/enter_question_mark.html)
| [从 Option 中取出 Result](./src/multiple_error_types/option_result.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/multiple_error_types/option_result.html)
| [定义一个错误类型](./src/multiple_error_types/define_error_type.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/multiple_error_types/define_error_type.html)
| [把错误 “装箱”](./src/multiple_error_types/boxing_errors.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/multiple_error_types/boxing_errors.html)
| [包裹错误](./src/multiple_error_types/wrap_error.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/multiple_error_types/wrap_error.html)

| [遍历 Result](./src/iter_result.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/error/iter_result.html)






## 章节小总

```
type Result<T> = std::result::Result<T, DoubleError>;

// 实现从 `ParseIntError` 到 `DoubleError` 的转换。
// 在使用 `?` 时，或者一个 `ParseIntError` 需要转换成 `DoubleError` 时，它会被自动调用。
//由于我们parse会抛出ParseIntError，当与返回类型的错误不匹配时尝试调用Form#from方法转换DoubleError
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}
```