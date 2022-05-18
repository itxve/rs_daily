#![allow(dead_code)]
///reference https://rustwiki.org/zh-CN/rust-by-example/custom_types/enum.html

// WebEvent
enum WebEvent {
    PageLoad,
    PageUnLoad,
    KeyPress(char),
    Click { x: i32, y: i32 },
}

fn dispatch(event: &WebEvent) {
    match event {
        WebEvent::PageLoad => {
            println!("PageLoad...")
        }
        WebEvent::PageUnLoad => {
            println!("PageUnLoad...")
        }
        WebEvent::KeyPress(char_str) => {
            println!("You KeyPress the [{}]", char_str)
        }
        WebEvent::Click { x: xx, y } => {
            println!("You  Click the x:[{}],y:[{}]", xx, y)
        }
    }
}
#[test]
fn prnt_enum() {
    let page_load = WebEvent::PageLoad;
    dispatch(&page_load);
    let page_un_load = WebEvent::PageUnLoad;
    dispatch(&page_un_load);
    let key_press = WebEvent::KeyPress('😁');
    dispatch(&key_press);
    let click = WebEvent::Click { x: 100, y: 120 };
    dispatch(&click);
}

/**
 * 使用use，减少前缀
 */
#[test]
fn use_enum() {
    use WebEvent::*;
    let page_load = PageLoad;
    dispatch(&page_load);
    let page_un_load = PageUnLoad;
    dispatch(&page_un_load);
    let key_press = KeyPress('😁');
    dispatch(&key_press);
    let click = Click { x: 100, y: 120 };
    dispatch(&click);
}

/**
 * 使用use，减少前缀
 */
#[test]
fn enum_value() {
    // 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
    // WebEvent中无法使用 as转换
    enum WebEventC {
        PageLoad,
        PageUnLoad,
    }

    // 拥有显式辨别值（explicit discriminator）的 enum
    enum Color {
        Red = 0xffffff,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    use Color::*;
    use WebEventC::*;
    let page_load = PageLoad;
    println!("zero is {}", page_load as u8);

    println!("Color is {}", Red as u16);
}
