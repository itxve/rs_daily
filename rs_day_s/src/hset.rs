use std::collections::HashSet;

#[test]
fn set_use() {
    let mut a: HashSet<i32> = (1..=7).into_iter().collect();
    let mut b: HashSet<i32> = (5..=10).into_iter().collect();

    // 若一个集合（collection）的元素类型实现了 `Debug`，那么该集合也就实现了 `Debug`。
    // 这通常将元素打印成这样的格式 `[elem1, elem2, ...]
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // 乱序打印 [1, 2, 3, 4, 5]。
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // 这将会打印出 [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // 乱序打印 [2, 3, 4]。
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );

    // 打印 [1, 5]
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}
