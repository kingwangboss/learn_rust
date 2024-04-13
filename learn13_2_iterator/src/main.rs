fn main() {
    println!("Hello, world!");
}
fn test1() {
    // 迭代器模式允许你对一个序列的项进行某些处理。迭代器（iterator）负责遍历序列中的每一项和决定
    // 序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑。
    // 在 Rust 中，迭代器是 惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果。
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got:{}", *val);
    }
}

fn test2() {
    // 迭代器都实现了一个叫做Iterator的定义标准库trait
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item> {
            // 此处省略了默认实现
            None
        }
    }
    // next 是 Iterator 实现者被要求定义的唯一方法。next 一次返回迭代器中的一个项，
    // 封装在 Some 中，当迭代器结束时，它返回 None。
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
    // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。
}

// 产生其他迭代器的方法
/* Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors），
他们允许我们将当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。
不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。 */
fn test3() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map( |x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

// 实现Iterator trait来创建自定义迭代器
// 创建一个只会从 1 数到 5 的迭代器
fn test4() {
    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter {
                count: 0,
            }
        }
    }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();

    for _ in 0..5 {
        println!("{}", counter.next().unwrap());
    }
}   
