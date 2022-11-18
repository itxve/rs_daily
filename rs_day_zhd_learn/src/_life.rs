use std::fmt::Debug;

fn return_str<'a>() -> &'a str {
    let mut ll = "12/".to_string();
    // 所有权无法移出
    // &ll[..]
    ""
}

//T: 'a：在 T 中的所有引用都必须比生命周期 'a 活得更长。
fn the_longest<'c, 'b: 'c, 'a: 'c>(s1: &'a str, s2: &'b str) -> &'c str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

#[test]
fn do_life() {
    let ab = "ab2";
    {
        let cd = "cd";
        let cs = the_longest(ab, cd);
        println!("cs: {}", cs);
    }
}

fn f<'a>() {}
fn g<'a: 'a>() {}
fn t<T>() {}
fn do_life_a() {
    // let fs = f::<'static> as fn(); //late bound
    let gs = g::<'static> as fn(); //early bound
    let ts = t::<u8> as fn(); //单态化
}

struct Buffer<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'b, 'a: 'b> Buffer<'a> {
    fn new(b: &'a [u8]) -> Buffer {
        Buffer { buf: b, pos: 0 }
    }

    fn read_bytes_early(&'b mut self) -> &'a [u8] {
        self.pos += 3;
        &self.buf[self.pos - 3..self.pos]
    }
    fn read_bytes_late<'c>(&'c mut self) -> &'c [u8] {
        self.pos += 3;
        &self.buf[self.pos - 3..self.pos]
    }
}
#[test]
fn do_browwer() {
    let mut vs = Buffer::new(&[1u8, 2, 23]);

    let sdw = vs.read_bytes_early();

    // 只能有一个可变借用
    // &'b 的 生命周期<= 'a的周期 下面可以继续借用
    let sdw = vs.read_bytes_early();
}

trait DoThing<T> {
    fn do_sth(&self, value: T);
}
impl<'a, T: Debug> DoThing<T> for &'a usize {
    fn do_sth(&self, value: T) {
        println!("{:?}", value)
    }
}

//高阶限定
fn try_do_sth<'a>(b: Box<dyn for<'f> DoThing<&'f usize>>) {
    let s: usize = 10;
    b.do_sth(&s);
}
#[test]
fn plus_life() {
    let gf = Box::new(&10usize);
    try_do_sth(gf);
}
