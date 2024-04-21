use std::rc::Rc;

fn main() {
    test2();
    test3();
}

// Rc<T> 引用计数智能指针
fn test1() {
    /* 
    大部分情况下所有权是非常明确的：可以准确地知道哪个变量拥有某个值。然而，
    有些情况单个值可能会有多个所有者。例如，在图数据结构中，多个边可能指向相同的节点，
    而这个节点从概念上讲为所有指向它的边所拥有。节点直到没有任何边指向它之前都不应该被清理。

    为了启用多所有权，Rust 有一个叫做 Rc<T> 的类型。其名称为 引用计数（reference counting）的缩写。
    引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，
    就代表没有任何有效引用并可以被清理。

    可以将其想象为客厅中的电视。当一个人进来看电视时，他打开电视。其他人也可以进来看电视。
    当最后一个人离开房间时，他关掉电视因为它不再被使用了。如果某人在其他人还在看的时候就关掉了电视，
    正在看电视的人肯定会抓狂的！

    Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的
    哪一部分会最后结束使用它的时候。如果确实知道哪部分是最后一个结束使用的话，就可以令其成为
    数据的所有者，正常的所有权规则就可以在编译时生效。

    注意 Rc<T> 只能用于单线程场景；第 16 章并发会涉及到如何在多线程程序中进行引用计数。
     */
}
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
// 使用 Rc<T> 共享数据
fn test2() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // 当创建 b 时，不同于获取 a 的所有权，这里会克隆 a 所包含的 Rc<List>，
    // 这会将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc<List> 中数据的所有权。
    let b = Cons(3, Rc::clone(&a));
    // 创建 c 时也会克隆 a，这会将引用计数从 2 增加为 3。每次调用 Rc::clone，
    // Rc<List> 中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理。
    let c = Cons(4, Rc::clone(&a));
    println!("c = {:#?}", c);
    /* 
    Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。
    Rc::clone 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。
    通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。
    当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
     */
}

// 克隆 Rc<T> 会增加引用计数
fn test3() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
