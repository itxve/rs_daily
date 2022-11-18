trait TraT {
    fn f(self);
}

impl<T> TraT for fn(T) {
    fn f(self) {
        println!("1")
    }
}

impl<T> TraT for fn(&T) {
    fn f(self) {
        println!("2")
    }
}

#[test]
fn do_t() {
    let af: fn(u8) = (|a: u8| println!("af"));
    let bf: fn(&u8) = (|a: &u8| println!("bf"));
    let cf: fn(_) = (|a: &u8| println!("cf"));
    af.f();
    bf.f();
    cf.f();
}
