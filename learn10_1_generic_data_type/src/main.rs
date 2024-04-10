/* 如果要在函数体中使用参数，就必须在函数签名中声明它的名字，好让编译器知道这个名字指代的是什么。
同理，当在函数签名中使用一个类型参数时，必须在使用它之前就声明它。为了定义泛型版本的 largest 函数，
类型参数声明位于函数名称与参数列表中间的尖括号 <> 中 */
// 可以这样理解这个定义：函数 largest 有泛型类型 T。它有个参数 list，
// 其类型是元素为 T 的 slice。largest 函数的返回值类型也是 T。
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         /* 注释中提到了 std::cmp::PartialOrd，这是一个 trait。下一部分会讲到 trait。
//         不过简单来说，这个错误表明 largest 的函数体不能适用于 T 的所有可能的类型。
//         因为在函数体需要比较 T 类型的值，不过它只能用于我们知道如何排序的类型。为了开启比较功能，
//         标准库中定义的 std::cmp::PartialOrd trait 可以实现类型的比较功能（
//         查看附录 C 获取该 trait 的更多信息）。 */
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// 结构体中的泛型
struct Point<T, U> {
    x: T,
    y: U,
}

// 枚举中的泛型
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 方法中的泛型
// 注意 Point<T, U> 的定义中只使用了一个泛型类型，这个定义表明结构体 Point<T, u> 对于一些类型 T 是泛型的，
// 而且字段 x 和 y 都是 相同类型的，无论它具体是何类型。
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

fn main() {
    let wont = Point{ x: 5, y: 4.0 };
    println!("p.x = {}, p.y = {}", wont.x(), wont.y());
}
