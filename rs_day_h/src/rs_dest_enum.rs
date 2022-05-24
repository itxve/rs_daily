#![allow(dead_code)]

#[derive(Debug)]
enum Color {
    // 这三个取值仅由它们的名字（而非类型）来指定。
    Red,
    Blue,
    Green,
    // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, f32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[test]
fn test_dest_enum() {
    use Color::{Red, RGBA};
    let rgba = RGBA(255, 0, 0, 0.5);
    let rgba = Red;
    match rgba {
        RGBA(r, g, b, a) => {
            println!("({},{},{},{})", r, g, b, a);
        }
        Red => {
            println!("is Red");
        }
        _ => {
            println!("orther is {:?}", rgba);
        }
    }
}
