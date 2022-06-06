#![allow(dead_code)]

mod display {

    fn the_private() {
        println!("I am private , called by the_public");
    }

    // 默认是私有的  使用pub关键字重写为公开
    pub fn the_public() {
        println!("I am public.");
        the_private();
    }

    // pub(in path)
    pub fn public_in_path() {
        netest::public_function_in_display();
        netest::public_function_in_super_mod();
        // 按道理来说编译器不应该有这个方法选项
        // netest::public_function_in_nested();
    }

    //嵌套的mod 也是需要选择性公开的 ,没有 pub ,是无法使用 display::netest调用的
    mod netest {
        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）,
        // 不能是兄弟模块
        pub(in crate::mod_visibility::display) fn public_function_in_display() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
        }

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }
}

#[test]
fn main() {
    //外面只能调用公开的方法
    display::the_public();
    display::public_in_path();
}
