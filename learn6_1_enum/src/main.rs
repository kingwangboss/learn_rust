// enum IpAddKind {
//     V4,
//     V6,
// }
// struct IpAddr {
//     kind: IpAddKind,
//     address: String,
// }
// 一种更简洁的方式表达相同概念
enum IpAddr {
    V4(String),
    V6(String),
}
// 标准库提供一个开箱即用的定义
struct Ipv4Addr {

}
struct Ipv6Addr {

}
// 我们可以将任意类型的数据放入枚举成员中：字符串，数字类型或者结构体，甚至可以包含另一个枚举。
enum IpAddr1 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move{ x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
// 枚举和结构体还有一个相似点：就像可以使用impl来为结构体定义方法那样，也可以在枚举上定义方法。
impl Message {
    fn call(&self) {
        println!("Message 方法");
    }
}

// Rust没有空值，但是它有一个可以编码存在或不存在概念的枚举，Option<T>,他在标准库中定义如下：
// enum Option<T> {
//     Some(T),
//     None,
// }
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option<T>枚举很有用，以至于它被包含在了prelude中，你不需要显示引入作用域。
    // 它的成员也是如此，不需要Option::前缀来使用Some和None。
    let some_number = Some(5);
    let some_string = Some("a string");
    // 如果使用None，必须要告诉编译器Option<T>是什么类型，因为编译器只通过None无法推断Some成员保存的类型
    let absent_number: Option<i32> = None;

    let x:i32 = 5;
    let y:Option<i32> = Some(5);
    // Option<T>和T是不同类型，这里不允许Option<T>和i32相加，编译器不允许向一个肯定有效的值那样使用Option<T>
    // 如x拥有一个i32类型的值，编译器确保它总是一个有效的值，我们可以自信使用，不需要做空值检查
    // 只有有当使用Option<i32>时需要担心可能没有值，而编译器会确保我们在使用值之前处理了为空的情况。
    // let sum = x + y;

}
