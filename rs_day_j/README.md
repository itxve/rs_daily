# [模块](https://rustwiki.org/zh-CN/rust-by-example/mod.html)

|        仓库文件索引          |         原文地址         |   
|----------------------|:-----------------------:|
| [可见性](./src/mod_visibility.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/mod/visibility.html)| 
| [结构体的可见性](./src/mod_struct_visibility.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/mod/struct_visibility.html)| 
| [use 声明](./src/mod_use.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/mod/use.html)| 
| [super 和 self](./src/mod_self_super.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/mod/super.html)|
| [模块文件分层](./src/mod_split/mod.rs) |  [原文](https://rustwiki.org/zh-CN/rust-by-example/mod/split.html)|




## 章节小总结

### crate::* 和lib.rs中导入的mod有关
 ```
 src/slib.rs
 //crate只暴露了声明的模块
 mod mod_self_super;
 mod mod_struct;
 ```

### mod *
```
// 此声明将会查找名为 `*.rs` 或 `*/mod.rs` 的文件，并将该文件的内容放到
// 此作用域中一个名为 `*` 的模块里面。
mod orther;
```
