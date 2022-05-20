#![allow(dead_code)]
///reference https://rustwiki.org/zh-CN/rust-by-example/custom_types/enum/testcase_linked_list.html

/// 定义一个Node 结构
#[derive(Debug)]
enum Node {
    // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(u32, Box<Node>),
    // Nil：末结点，表明链表结束
    Nil,
}

impl Node {
    // Self指代Node本身
    fn new() -> Self {
        Self::Nil
    }

    fn add(self, id: u32) -> Self {
        Self::Cons(id, Box::new(self))
    }

    // 返回 List 的长度
    fn len(&self) -> u32 {
        // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的
        // 取值种类。
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
        // 类型要好过匹配引用 `&T`。
        // 这个目前不是特别理解 【???】

        // *self解引用
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；
            // 因此使用一个对 tail 的引用
            // ref 通过引用绑定 ref==& [???]
            // [tail.len()]好像差不多像递归调用
            Self::Cons(_, ref tail) => 1 + tail.len(),
            _ => 0,
        }
    }

    // 返回列表的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        match *self {
            Self::Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                // 而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            }
            Self::Nil => {
                format!("Nil")
            }
        }
    }
}

#[test]
fn test_node() {
    // 创建一个空链表
    let mut node = Node::new();

    // // 追加一些元素
    node = node.add(1);
    node = node.add(5);
    node = node.add(8);
    node = node.add(14);

    println!("node：{:?}", node);
    // // 显示链表的长度
    println!("linked list has length: {}", node.len());
    //stringify
    println!("stringify:{}", node.stringify());
}
