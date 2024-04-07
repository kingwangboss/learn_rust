fn test1() {
    let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }
    // 当我们只想要对Some(3)匹配进行操作但是不想处理任何其他模式时，为了满足match表达式穷尽性要求，
    // 我们在后面加上了_ => ()，这样有点多余。
    // 我们可以使用if let这种更短的方式编写
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        // else其实就是match表达式中的_分支
    }
    // 可以认为if let 就是match的一个语法糖
}

fn main() {
    test1();
}
