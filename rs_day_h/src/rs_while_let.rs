#![allow(dead_code)]

#[test]
fn test_while_let() {
    let op = Some(10);
    let mut i = 0;
    while true {
        match op {
            Some(n) => {
                if i == 10 {
                    println!("n: {} ,i :{} ", n, i);
                    break;
                } else {
                    i += 1;
                }
            }
            None => {}
        }
    }
    i = 0;
    // 代码清爽许多
    while let Some(n) = op {
        if i == 20 {
            println!("n: {} ,i :{} ", n, i);
            break;
        }
        i += 1;
    }
}
