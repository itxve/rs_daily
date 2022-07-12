fn create_box() {
    // 在堆上分配一个整型数据
    let _box1 = Box::new(3);

    // `_box1` 在这里被销毁，内存得到释放
}
struct Ky(i32);

// rustc raii.rs && valgrind ./raii -v
// mac 版的valgrind貌似有点毛病
// total heap usage: 0 allocs, 0 frees, 0 bytes allocated
// #[test]
fn main() {
    // 在堆上分配一个整型数据
    let _box2 = Box::new(5i32);

    // 嵌套作用域：
    {
        // 在堆上分配一个整型数据
        let _box3 = Box::new(4);

        // `_box3` 在这里被销毁，内存得到释放
    }

    // 创建一大堆 box（只是因为好玩）。
    // 完全不需要手动释放内存！
    for i in 0..1_0000 {
        create_box();
        Ky(i);
    }

    // `_box2` 在这里被销毁，内存得到释放w
}
