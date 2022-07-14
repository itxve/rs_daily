trait Fly1 {
    fn fly(&self);
}
trait Fly2 {
    fn fly(&self);
}

struct TestFly;

impl Fly1 for TestFly {
    fn fly(&self) {
        println!("{}", "fly 1")
    }
}

impl Fly2 for TestFly {
    fn fly(&self) {
        println!("{}", "fly 2")
    }
}

#[test]
fn main() {
    let v = TestFly;
    Fly1::fly(&v);
    // <TestFly as Fly1>::fly(&v);
    Fly2::fly(&v);
    // <TestFly as Fly2>::fly(&v);
}
