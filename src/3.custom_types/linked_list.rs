/*
enum的一个常见用途是用来创建一个链表（linked list）
*/

use List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    // Cons： 包装了元素和指向下个节点的指针的元组结构体
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    // Nil： 标志着该链表结束的节点
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    // 创建空链表
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    // 在链表前面加一个元素并返回整个链表
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    // 返回链表的长度
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // `self`必须匹配，因为此方法的行为依赖于`self`变量
        // `self`有类型`&List`, `*self`有类型`List`，固定类型`T`匹配引用`&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            // 不能拿走tail的所有权，因为`self`是借用的
            // 可以用tail的引用代替
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    // 返回链表（堆分配）的字符串表示
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                // `format!`类似于`print!`，但是可以在控制台返回堆分配的字符串
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Append some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
