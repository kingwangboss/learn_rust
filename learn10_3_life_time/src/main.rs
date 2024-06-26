
// 生命周期避免了悬垂引用 
fn test1() {
    let r = 1;
    {
        let x = 5;
        // r = &x;
        // 不能使用离开作用域的值的引用
        // Rust编译器有个借用检查器，它比较作用域来确保所有的借用都是有效的。
    }
    println!("r: {}", r);
}
/* 提示文本揭示了返回值需要一个泛型生命周期参数，因为 Rust 并不知道将要返回的引用是指向 x 或 y。
事实上我们也不知道，因为函数体中 if 块返回一个 x 的引用而 else 块返回一个 y 的引用！
当我们定义这个函数的时候，并不知道传递给函数的具体值，所以也不知道到底是 if 还是 else 会被执行。
我们也不知道传入的引用的具体生命周期，借用检查器自身同样也无法确定，因为它不知道 x 和 y 的生命周期
是如何与返回值的生命周期相关联的。为了修复这个错误，我们将增加泛型生命周期参数来定义引用间的关系以便
借用检查器可以进行分析。 */
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// 函数签名中的生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/* 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 
输出生命周期（output lifetimes）。
编译器采用三条规则来判断引用何时不需要明确的标注。
第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。
如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，
编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。
第一条规则是每一个是引用的参数都有它自己的生命周期参数。换句话说就是，
有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数
的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期
参数：fn foo<'a>(x: &'a i32) -> &'a i32。
第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
说明是个对象的方法(method) 那么所有输出生命周期参数被赋予 self 的生命周期。
第三条规则使得方法更容易读写，因为只需更少的符号。 */
fn main() {
    let str1 = String::from("abcd");
    let str2 = "xyz";

    let result = longest(str1.as_str(), str2);
    println!("result: {}", result);

    // 'static，其生命周期能够存活于整个程序期间。所有的字符串字面量都拥有 'static 生命周期.
    let s: &'static str = "I have static lifetime";

}