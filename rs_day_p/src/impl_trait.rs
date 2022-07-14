#![allow(dead_code)]

use std::iter;
use std::vec::IntoIter;

//使用别名缩短
type ShortItor = iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>>;

// 该函数组合了两个 `Vec <i32>` 并在其上返回一个迭代器。
// 看看它的返回类型多么复杂！
fn combine_vecs_explicit_return_type(v: Vec<i32>, u: Vec<i32>) -> ShortItor {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 这是完全相同的函数，但其返回类型使用 `impl Trait`。
// 看看它多么简单！
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

struct Vc(i32);
trait Myta {
    fn note(&self) -> i32;
}
impl Myta for Vc {
    fn note(&self) -> i32 {
        self.0
    }
}

fn note_myta() -> impl Myta {
    Vc(123)
}

// 返回一个将输入和 `y` 相加的函数
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

// 返回一个将输入的数组和 `y` 相累加的函数
fn sum_fc(y: i32) -> impl Fn(&Vec<i8>) -> i32 {
    let closure = move |xvc: &Vec<i8>| {
        let mut rt: i32 = 0;
        for &idx in xvc.into_iter() {
            rt += idx as i32;
        }
        rt + y
    };
    closure
}

#[test]
fn main() {
    let fr = note_myta();
    println!("myta ==:{}", fr.note());

    let plus_one = make_adder_function(1);
    println!("plus_one::: {}", plus_one(100));

    let fc_sum = sum_fc(5);
    println!("fc_sum::: {}", fc_sum(&vec![2, 3, 4]));

    let v1 = vec![1];
    let v2 = vec![2];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(1), v3.next());
    println!("all done");
}
