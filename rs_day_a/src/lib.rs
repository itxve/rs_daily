///
/// 描述下这个方法
///
/// # example
/// ```
/// println!("{}", "在控制台输出")
/// ```
/// # 参考fmt模块
/// reference https://doc.rust-lang.org/std/fmt/index.html
///
pub fn pln() {
    println!("{}", "I am rs_day_one")
}

mod display;

#[cfg(test)]
mod tests {
    use crate::pln;

    #[test]
    fn pln_test() {
        pln();
    }

    #[test]
    fn prnt() {
        // 通常情况下，`{}` 会被任意变量内容所替换。
        // 变量内容会转化成字符串。
        // {}数量与参数数量一直
        println!("{} {}days", 31, 12);
    }
    #[test]
    fn prnt_index() {
        // 使用索引站位
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    }

    #[test]
    fn prnt_name() {
        // 使用索引站位
        println!(
            "{a}, {a}. {b}, this is {c}",
            a = "A-k",
            b = "B-K",
            c = "AliceBob"
        );
    }

    // 使用二进制类型打印
    #[test]
    fn prnt_binary() {
        println!("{} of {:b} people know binary, the other half don't", 1, 2);
    }

    #[test]
    fn prnt_width() {
        // 你可以在数字左边补 0。下面语句输出 "001"。
        //width  是总长度
        println!("{number:0width$}", number = 1, width = 3);
    }

    //一个宏 实现Debug，可以用于打印
    #[derive(Debug)]
    struct DebugPrintable(i32);

    //一个宏 实现Debug，可以用于打印
    #[derive(Debug)]
    struct DebugPrintableBeatuty {
        name: String,
        age: u8,
    }

    #[test]
    fn prnt_debug() {
        // 没有实现Display 只能使用{:?} 打印，{:#?} 是美化版的
        println!("{:?}", DebugPrintable(123));

        //DebugPrintableBeatuty { name: "Beatuty", age: 18 }
        println!(
            "{:?}",
            DebugPrintableBeatuty {
                name: String::from("Beatuty"),
                age: 18
            }
        );

        // DebugPrintableBeatuty {
        //     name: "Beatuty",
        //     age: 18,
        // }
        println!(
            "{:#?}",
            DebugPrintableBeatuty {
                name: String::from("Beatuty"),
                age: 18
            }
        );
    }
}
