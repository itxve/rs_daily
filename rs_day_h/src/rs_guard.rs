#![allow(dead_code)]

#[test]
fn test_guard() {
    match_vec((10, 2));
    match_vec((100, 2));
    match_vec((25, 4));
    match_vec((50, 2));
    match_vec((20, 10));
}
// match 匹配只能进入一个分支，我们使用【卫语句】条件判断，选择不同分支
fn match_vec(tp: (u16, u16)) {
    match tp {
        (1..=10, 1..=2) => {
            println!("fisrt in 1-10,second in 1-2")
        }
        (100, x) if x == 2 => {
            println!("(100,2)")
        }
        (f, s) if f * s == 100 => {
            println!("f: {} ,s: {} ,f * s = {}", f, s, 100)
        }
        // 最后两个分支是等价的
        // (f, s) => {
        //     println!("f: {} ,s: {} ", f, s);
        // }
        _ => {
            println!("orther :{:?}", tp)
        }
    }
}
