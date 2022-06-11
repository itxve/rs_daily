// 这个函数仅当目标系统是 Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// 而这个函数仅当目标系统 **不是** Linux 时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

#[test]
fn main() {
    use std::env::consts::OS;
    println!("my system os is : [ {} ]", OS);
    are_you_on_linux();
    if cfg!(target_os = "macos") {
        println!("You System is Macos");
    }
}
