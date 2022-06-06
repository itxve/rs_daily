#![allow(dead_code)]

#[derive(Debug)]
struct Hn {
    name: String,
}

pub fn public_function() {
    println!(
        "called rary's `public_function()` : {:?}",
        Hn {
            name: String::from("name")
        }
    );
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");
    private_function();
}
