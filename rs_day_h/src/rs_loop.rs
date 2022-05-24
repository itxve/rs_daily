#![allow(dead_code)]

#[test]
fn test_loop() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");
            // 跳过这次迭代的剩下内容
            continue;
        }

        println!("{}", count);

        //255
        if count == u8::MAX as u32 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }
}

#[test]
fn test_loop_label() {
    let mut a = 0;
    'a: loop {
        a += 2;
        let mut b = 0;
        'b: loop {
            b += 1;
            if b > 100 {
                break 'b;
            }
            if a == 100 && b == 50 {
                println!("a:{},b:{}", a, b);
                break 'a;
            }
        }
    }
}

#[test]
fn test_loop_return() {
    let mut c = 0;
    let count100 = loop {
        c += 2;
        if c >= 100 {
            break c;
        }
        if c % 20 == 0 {
            break c * 4;
        }
    };
    println!("count100={}", count100)
}
