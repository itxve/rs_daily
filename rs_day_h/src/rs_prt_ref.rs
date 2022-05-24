#![allow(dead_code)]

#[test]
fn test_dest_reference() {
    // 获得一个 `i32` 类型的引用。`&` 表示取引用。
    let mut reference = &4;

    match reference {
        // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
        // `&i32`（译注：即 `reference` 的类型）
        // `&val`（译注：即用于匹配的模式）
        // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
        // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
        &val => println!("Got a value via destructuring: {}", val),
    }

    // 如果不想用 `&`，需要在匹配前解引用。
    match reference {
        ref mut val => {
            println!("ref mut val: {}", val);
            *val = &180;
        }
    }

    // 如果不想用 `&`，需要在匹配前解引用。
    match *reference {
        val => {
            println!("*reference : {}", val);
        }
    }
    println!("reference is: {}", reference);

    // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句
    // 的右边已经是一个引用。但下面这个不是引用，因为右边不是。
    let _not_a_reference = 3;

    match _not_a_reference {
        3 => {
            println!("_not_a_reference :{}", _not_a_reference)
        }
        _ => {
            println!("orther _not_a_reference :{}", _not_a_reference)
        }
    }

    let ref mut rf = -123;
    *rf = 123;
    println!("rf :{}", rf);

    // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
    let value = 5;

    // 使用 `ref` 关键字来创建引用。
    // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
    // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
    // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
    // 引用。
    match value {
        ref r => println!("Got a reference to a value: {:?}", *r),
    }
}

#[test]
fn test_dest_reference_mut() {
    let mut mut_value = 6;
    // 类似地使用 `ref mut`。
    match mut_value {
        ref mut m => {
            // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
            //这里的m 看作是mut_value，变更m就是变更mut_value
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    println!("v: {:?}", mut_value);
}
