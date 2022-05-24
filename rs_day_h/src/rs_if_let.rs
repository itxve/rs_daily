#![allow(dead_code)]

enum Event {
    OnLoad,
    Click(i16, i16),
    LongTap(u8),
}

#[test]
fn test_if_let() {
    let s = None;
    let ff = if let Some(number) = s {
        println!("{}", number);
        number
    } else {
        -100
    };
    println!("ff :{}", ff);

    let click = Event::Click(10, 100);
    let long_tap = Event::LongTap(3);
    let on_load = Event::OnLoad;
    if let Event::Click(x, y) = click {
        println!("x: {}, y: {}", x, y)
    }

    if let Event::LongTap(time) = long_tap {
        println!("Long Tap : {}s", time)
    }

    // 未实现PartialEq
    // if on_load == Event::OnLoad {}
    if let Event::OnLoad = on_load {
        println!("OnLoad...")
    }
}
