use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // 生成一个hello.rs
    let dest_path = Path::new(&out_dir).join("hello.rs");
    let mut f = File::create(&dest_path).unwrap();
    //写入文件内容
    f.write_all(
        b"
        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
    ",
    )
    .unwrap();
}
