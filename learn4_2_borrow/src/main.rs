// 1.在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
// 2.引用必须总是有效的。
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("length: {}, s1: {}", len, s1);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2: {s2}");

    ref_mutref();

    println!("dangle: {}", dangle());
}

// &符号就是引用，它们允许你使用值但不获取其所有权，我们将创建一个引用的行为称为借用。
fn calculate_length(s: &String) -> usize {  // s是对String的引用
    s.len()
}//这里s离开作用域，但因为它并不拥有引用值的所有权
// 所有这里不会drop

// 当我们需要修改借用的值需要使用可变引用
// 不过可变引用有个很大的限制：在同一时间，只能有一个对某一特定数据的可变引用，Rust可以在编译避免数据竞争。
// 数据竞争由这三个行为同时造成：
// 1.两个或多个指针访问同一数据。
// 2.至少有一个指针被用来写入数据。
// 3.没有同步数据访问的机制。
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 引用和可变引用不能同时使用
fn ref_mutref() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1}{r2}");
    // 此位置后r1,r2不再使用

    let r3 = &mut s;
    println!("{r3}");
}

// 解决悬垂引用,如果这里返回值为&String就会有悬垂引用，创建的s作用域在dangle函数，
// 所有必须返回String把所有权转移出去
fn dangle() -> String {
    let s = String::from("hello");
    s
}