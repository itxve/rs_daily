include!(concat!(env!("OUT_DIR"), "/hello.rs"));

#[test]
fn main() {
    println!("{} ,{}", message(), env!("OUT_DIR"));
}
