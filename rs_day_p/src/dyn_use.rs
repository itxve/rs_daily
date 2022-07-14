#![allow(dead_code)]

struct Sheep {
    time: u8,
}
struct Cow {
    weight: u16,
}

trait Animal {
    // 实例方法签名
    fn noise(&self) -> u16;
}

// 实现 `Sheep` 的 `Animal` trait。
impl Animal for Sheep {
    fn noise(&self) -> u16 {
        self.time.into()
    }
}

// 实现 `Cow` 的 `Animal` trait。
impl Animal for Cow {
    fn noise(&self) -> u16 {
        self.weight
    }
}

// 返回一些实现 Animal 的结构体，但是在编译时我们不知道哪个结构体。
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep { time: 10 })
    } else {
        Box::new(Cow { weight: 55 })
    }
}
#[test]
fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
