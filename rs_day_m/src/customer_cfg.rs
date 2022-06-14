#![allow(dead_code)]

#[cfg(fa)]
fn conditional_function() {
    println!("condition met!")
}

#[cfg(fb = "cd")]
fn conditional_function2() {
    println!("condition met enable!")
}

// rustc customer_cfg.rs --cfg  'fb="cd"' --cfg fa && ./customer_cfg

fn main() {
    conditional_function();
    conditional_function2();
}
