fn main() {
    test1();
}

// 使用 Drop Trait 运行清理代码
fn test1() {
    struct CustomSmartPointer {
        data: String,
    }
    /* 
    在 Rust 中，可以指定每当值离开作用域时被执行的代码，编译器会自动插入这些代码。
    于是我们就不需要在程序中到处编写在实例结束时清理这些变量的代码 —— 而且还不会泄漏资源。
    指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。Drop trait 要求实现一个叫
    做 drop 的方法，它获取一个 self 的可变引用。为了能够看出 Rust 何时调用 drop，让我们暂时
    使用 println! 语句实现 drop。 */
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }
    
    
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    // 通过 std::mem::drop 提早丢弃值
    drop(d);
    println!("CustomSmartPointers created.");
}
