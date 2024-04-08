/* vector允许我们在一个单独的数据结构中存储多个值，所有值在内存中彼此相邻排列。vector只能储存相同类型
的值。它们在拥有一系列项的场景下非常实用。 */

fn main() {
    // 新建vector
    let mut v1:Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    // 更新vector
    v1.push(4);
    println!("v1: {:?}, v2: {:?}",v1, v2);

    // 读取vector元素
    let v3 = vec![1, 2, 3, 4, 5];
    // 直接通过索引访问这里，third 会得到 v3 中索引为 2 的元素的一个副本。
    // 如果元素类型实现了 Copy trait（如 i32、f64 等基本类型），这种方式是最简单和直接的。
    let third = v3[2];
    // 使用引用,如果你想避免复制
    // （可能因为类型没有实现 Copy trait，或者你想避免复制的成本），你可以使用引用：
    let four = &v3[3];
    // 这里的 four 是一个指针类型，指向 v3 的一个元素，但在打印时，
    // Rust 自动解引用它，因此你看到的是它指向的值，而不是内存地址, 写four和*four是一样的
    println!("第三个元素是：{}，第四个元素是：{}", third, *four);

    // 通过get读取vector的元素返回一个Option<&T>枚举
    match v3.get(2) {
        Some(third) => println!("third:{}", *third),
        None => println!("none"),
    }


    let mut v4 = vec![1, 2, 3, 4];
    let first = v4[0];
    // 这里first是指向v4第一个元素的不可变引用，下面对v4操作增加元素，这是不行的
    // 违背了Rust借用规则：在拥有某些内容的不可变或可变引用时，你不能再对这些内容进行修改。
    // let first = &v4[0];
    v4.push(111);
    println!("first：{:?}", first);

    // 遍历vector
    let v5 = vec![100, 32, 57];
    for item in &v5 {
        println!("item: {}", *item);
    }

    let mut v6 = vec![100, 32, 57];
    for item in &mut v6 {
        *item += 50;
    }
    println!("v6:{:?}", v6);

    // 通过枚举和vector存储不同类型的值
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    println!("row: {:#?}", row);
}
