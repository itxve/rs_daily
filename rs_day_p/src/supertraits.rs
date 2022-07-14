trait Person {
    type Item;
    fn name(&self) -> Self::Item;
}

// Person 是 Student 的父 trait。
// 实现 Student 需要你也 impl 了 Person。
trait Student: Person<Item = bool> {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student，计算机科学的学生) 是 Programmer 和 Student 两者的子类。
// 实现 CompSciStudent 需要你同时 impl 了两个父 trait。
trait CompSciStudent: Programmer + Student {
    type Item2;
    fn git_username(&self) -> Self::Item2;
}

#[derive(Clone)]
struct CompImpl(String);

impl CompSciStudent for CompImpl {
    type Item2 = bool;

    fn git_username(&self) -> Self::Item2 {
        true
    }
}

impl Programmer for CompImpl {
    fn fav_language(&self) -> String {
        self.0.clone()
    }
}

impl Student for CompImpl {
    fn university(&self) -> String {
        self.0.clone()
    }
}
impl Person for CompImpl {
    type Item = bool;
    fn name(&self) -> bool {
        false
    }
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent<Item2 = bool, Item = bool>) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

#[test]
fn main() {
    let ipmg = CompImpl(String::from("Tre"));
    let result = comp_sci_student_greeting(&ipmg);
    println!("{}", result)
}
