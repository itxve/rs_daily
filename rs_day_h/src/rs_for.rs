#![allow(dead_code)]

#[test]
fn test_for() {
    //区间标记 range
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        if n > 60 {
            break;
        }
    }
}

#[test]
fn test_iterator() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        //name是借用过来的，names可以继续使用
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    for name in names.iter() {
        println!("{}", name);
    }
}

#[test]
fn test_into_iterator() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        //name是借用过来的，names可以继续使用
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    //这里会提示，names已经被moved
    // for name in names.iter() {
    //     println!("{}", name);
    // }
}

#[test]
fn test_mut_iterator() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        match name {
            //匹配一个Item
            &mut "Bob" => {
                //修改item的数据
                *name = "BobPlus";
                //这种方式不能修改Item ,[???]
                // let mut c = *name;
                // c = "BobPlus";
            }
            _ => println!("Hello {}", name),
        }
    }
    for name in names.iter() {
        println!("{}", name);
    }
}
