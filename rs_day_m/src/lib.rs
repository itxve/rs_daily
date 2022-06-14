// 这个 crate 是一个库文件
#![crate_type = "lib"]
// 库的名称为 qw
#![crate_name = "qw"]

mod cfg;
mod cfg_attr;
mod customer_cfg;
mod dead_code;

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
