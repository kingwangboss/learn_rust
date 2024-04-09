/* 常用集合类型是 哈希 map（hash map）。HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。
它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。很多编程语言支持这种
数据结构，不过通常有不同的名字：哈希、map、对象、哈希表或者关联数组。 */

use std::collections::HashMap;

fn main() {
    // 新建hash map用new创建
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    // 新建hash map通过vector创建
    // 使用一个元组的 vector 的 collect 方法，其中每个元组包含一个键值对。
    // collect 方法可以将数据收集进一系列的集合类型，包括 HashMap。
    // 例如，如果队伍的名字和初始分数分别在两个 vector 中，可以使用 zip 方法来创建一个元组的 vector，
    // 其中 “Blue” 与 10 是一对，依此类推。接着就可以使用 collect 方法将这个
    // 元组 vector 转换成一个 HashMap.
    // 这里 HashMap<_, _> 类型标注是必要的，因为 collect 有可能当成多种不同的数据结构，
    // 而除非显式指定否则 Rust 无从得知你需要的类型。但是对于键和值的类型参数来说，可以使用下划线占位，
    // 而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型。
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores).collect();
    println!("scores: {:?}", scores2);

    // 获取hash map的值 get返回Option<V>
    let key = String::from("Blue");
    let score = scores2.get(&key);
    println!("score:{:?}", score);

    for (key, value) in &scores2 {
        println!("{}:{}", key, value);
    }

    
    
}

// hash map和所有权
fn test1() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // println!("{}", field_name);
}

// 更新hash map只有键没有值的情况
fn test2() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // 使用 entry 方法只在键没有对应一个值时插入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}

// 根据旧值更新一个值
fn test3() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);
}
