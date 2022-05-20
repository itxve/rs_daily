## [类型系统](https://rustwiki.org/zh-CN/rust-by-example/types.html)

|        仓库文件索引          |         原文地址         |        
|----------------------|:-----------------------------:|
| [类型转换](./src/rs_cast.rs)    |[原文](https://rustwiki.org/zh-CN/rust-by-example/types/cast.html)
| [变量字面量](./src/rs_literals.rs)  | [原文](https://rustwiki.org/zh-CN/rust-by-example/types/literals.html) 
| [类型推断](./src/rs_infer.rs)  | [原文](https://rustwiki.org/zh-CN/rust-by-example/types/inference.html) 
| [别名alias](./src/rs_alias.rs)  | [原文](https://rustwiki.org/zh-CN/rust-by-example/types/alias.html) 





#### 有符号整数类型（signed integer）

|         类型         |         占用内存         |         范围         |
|----------------------|:-----------------------:|:-----------------------:|
| i8   |  8 位（即 1 个字节）    |  [-2^7, 2^7-1]       
| i16  |  16 位（即 2 个字节）   |  [-2^15, 2^15 - 1]   
| i32  |  32 位（即 4 个字节）   |  [-2^31, 2^31 - 1]   
| i64  |  64 位（即 8 个字节）   |  [-2^63, 2^63 - 1]   
| i128 |  128 位（即 16 个字节） |  [-2^127, 2^127 - 1] 
| isize|  根据平台决定存储位数，在 32 位平台下占用 32 位，在 64 位平台下占用 64 位 


#### 无符号整数类型（unsigned integer）

|         类型         |         占用内存         |         范围         |
|----------------------|:-----------------------:|:-----------------------:|
| u8   |  8 位（即 1 个字节）    |  [0, 2^8]       
| u16  |  16 位（即 2 个字节）   |  [0, 2^16]   
| u32  |  32 位（即 4 个字节）   |  [0, 2^32]   
| u64  |  64 位（即 8 个字节）   |  [0, 2^64]   
| u128 |  128 位（即 16 个字节） |  [0, 2^128] 
| usize|  根据平台决定存储位数，在 32 位平台下占用 32 位，在 64 位平台下占用 64 位 





#### 参考

[Rust 自定义类型如何转换](../rs_day_f/)

[原码反码补码](https://www.cnblogs.com/zhangziqiu/archive/2011/03/30/computercode.html)

