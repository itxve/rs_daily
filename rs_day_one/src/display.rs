use std::fmt::{Binary, Display};

enum Unit {
    M(f32),
    CM(u16),
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Self=Unit
            Self::M(v) => f.write_fmt(format_args!(
                "我的长度是{}M
            ",
                v
            )),
            Self::CM(v) => f.write_fmt(format_args!(
                "我的长度是{}CM
          ",
                v
            )),
        }
    }
}

struct Cutom {
    height: Unit,
    age: u8,
}

//自己实现Display
impl Display for Cutom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "age==>{}\nUnit==>{}
        ",
            self.age, self.height
        ))
    }
}
impl Binary for Cutom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Cutom+Binary{}", 8))
    }
}

#[test]
fn custom_display() {
    // 只有实现Display才能使用{} 打印
    println!(
        "{}",
        Cutom {
            height: Unit::CM(186),
            age: 44
        }
    );

    println!(
        "{}",
        Cutom {
            height: Unit::M(1.86),
            age: 24
        }
    );

    println!(
        "{:b}",
        Cutom {
            height: Unit::M(1.86),
            age: 24
        }
    );
    //不出意外我会用 #[derive(Debug)]去打印  懒的
}

// 定义一个包含单个 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = &self.0;
        write!(f, "[")?;
        for (index, ele) in list.iter().enumerate() {
            if index != 0 {
                write!(f, " 🐰 ")?
            }
            write!(f, "{}", ele)?;
        }
        write!(f, "]")
    }
}
#[test]
fn prnt_list() {
    //[1 🐰 3 🐰 99]
    println!("{}", List(vec![1, 3, 99,]));
}

//定义一个结构体RGB
struct RGB(u8, u8, u8);

impl Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //:02 站2个位置，不足用O补足
        //:x 输出十六进制
        f.write_fmt(format_args!("0x{:02x}{:02x}{:02x}", self.0, self.1, self.2))
    }
}

// RGB (128, 255, 90) 0x80FF5A
// RGB (0, 3, 254) 0x0003FE
// RGB (0, 0, 0) 0x000000
#[test]
fn prnt_rgb() {
    println!("{}", RGB(128, 255, 90));
    println!("{}", RGB(0, 3, 254));
    println!("{}", RGB(0, 0, 0));
}
