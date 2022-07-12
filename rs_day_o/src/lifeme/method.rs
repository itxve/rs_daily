struct Owner(i32);

impl Owner {
    // 标注生命周期，就像独立的函数一样。
    fn add_one<'a>(&'a mut self, x: &i32) {
        println!("`print_multi`: x is {},", x);
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

#[test]
fn main() {
    let mut owner = Owner(18);
    owner.add_one(&12);
    owner.print();
}
