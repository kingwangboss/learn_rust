fn main() {
    // test1();
    test2();
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// 制造引用循环
fn test1() {
    /* 
    这里在变量 a 中创建了一个 Rc<List> 实例来存放初值为 5, Nil 的 List 值。
    接着在变量 b 中创建了存放包含值 10 和指向列表 a 的 List 的另一个 Rc<List> 实例。

    最后，修改 a 使其指向 b 而不是 Nil，这就创建了一个循环。为此需要使用 tail 方法
    获取 a 中 RefCell<Rc<List>> 的引用，并放入变量 link 中。接着使用 RefCell<Rc<List>> 的
     borrow_mut 方法将其值从存放 Nil 的 Rc<List> 修改为 b 中的 Rc<List>。
     */
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    /* 
    可以看到将列表 a 修改为指向 b 之后， a 和 b 中的 Rc<List> 实例的引用计数都是 2。
    在 main 的结尾，Rust 首先丢弃变量 b，这会使 b 中 Rc<List> 实例的引用计数减 1。
    然而，因为 a 仍然引用 b 中的 Rc<List>，Rc<List> 的引用计数是 1 而不是 0，所以 b 中
    的 Rc<List> 在堆上的内存不会被丢弃。接下来 Rust 会丢弃 a，同理这会将 a 中 Rc<List> 
    实例的引用计数从 2 减为 1。这个实例的内存也不能被丢弃，因为其他的 Rc<List> 实例仍在引用它。
    这些列表的内存将永远保持未被回收的状态。
     */

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

use std::rc::Weak;
// 创建树形数据结构：带有子节点的 Node
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// 避免引用循环：将 Rc<T> 变为 Weak<T>
fn test2() {
    /* 
    到目前为止，我们已经展示了调用 Rc::clone 会增加 Rc<T> 实例的 strong_count，和只在其 
    strong_count 为 0 时才会被清理的 Rc<T> 实例。你也可以通过调用 Rc::downgrade 并传递 
    Rc<T> 实例的引用来创建其值的 弱引用（weak reference）。调用 Rc::downgrade 时会得到 
    Weak<T> 类型的智能指针。不同于将 Rc<T> 实例的 strong_count 加1，调用 Rc::downgrade 
    会将 weak_count 加1。Rc<T> 类型使用 weak_count 来记录其存在多少个 Weak<T> 引用，类似于
     strong_count。其区别在于 weak_count 无需计数为 0 就能使 Rc<T> 实例被清理。

    强引用代表如何共享 Rc<T> 实例的所有权，但弱引用并不属于所有权关系。他们不会造成引用循环，
    因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断。

    因为 Weak<T> 引用的值可能已经被丢弃了，为了使用 Weak<T> 所指向的值，我们必须确保其值仍然有效。
    为此可以调用 Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>。如果 Rc<T> 值还未被丢弃，
    则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None。因为 upgrade 返回一个 Option<T>，我们确信 
    Rust 会处理 Some 和 None 的情况，所以它不会返回非法指针。
     */
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

// 可视化 strong_count 和 weak_count 的改变
fn test3() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    /* 
    一旦创建了 leaf，其 Rc<Node> 的强引用计数为 1，弱引用计数为 0。在内部作用域中创建了 branch 并
    与 leaf 相关联，此时 branch 中 Rc<Node> 的强引用计数为 1，弱引用计数为 1（因为 leaf.parent 
    通过 Weak<Node> 指向 branch）。这里 leaf 的强引用计数为 2，因为现在 branch 的 branch.children 
    中储存了 leaf 的 Rc<Node> 的拷贝，不过弱引用计数仍然为 0。

    当内部作用域结束时，branch 离开作用域，Rc<Node> 的强引用计数减少为 0，所以其 Node 被丢弃。
    来自 leaf.parent 的弱引用计数 1 与 Node 是否被丢弃无关，所以并没有产生任何内存泄漏！

    如果在内部作用域结束后尝试访问 leaf 的父节点，会再次得到 None。在程序的结尾，leaf 中 Rc<Node>
     的强引用计数为 1，弱引用计数为 0，因为现在 leaf 又是 Rc<Node> 唯一的引用了。

    所有这些管理计数和值的逻辑都内建于 Rc<T> 和 Weak<T> 以及它们的 Drop trait 实现中。通过在 Node 
    定义中指定从子节点到父节点的关系为一个Weak<T>引用，就能够拥有父节点和子节点之间的双向引用而不会
    造成引用循环和内存泄漏。
     */
}