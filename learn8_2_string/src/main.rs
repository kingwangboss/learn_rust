/* 字符串还是很复杂的。不同的语言选择了不同的向开发者展示其复杂性的方式。Rust 
选择了以准确的方式处理 String 数据作为所有 Rust 程序的默认行为，这意味着开发者们必须
更多的思考如何预先处理 UTF-8 数据。这种权衡取舍相比其他语言更多的暴露出了字符串的复杂性，
不过也使你在开发生命周期后期免于处理涉及非 ASCII 字符的错误。 
这引起了关于 UTF-8 的另外一个问题：从 Rust 的角度来讲，事实上有三种相关方式可以理解字符串：
字节、标量值和字形簇（最接近人们眼中 字母 的概念）。*/
fn main() {
    // 新建字符串
    let mut s1 = String::new();
    // 下面创建init字符串是一样的效果
    let data = "init";
    let s = data.to_string();
    let s = "init".to_string();
    let s = String::from("init");

    // 更新字符串
    // 使用push_str和push附加字符串
    let mut s2 = String::from("foo");
    s2.push_str("bar");
    println!("s2:{}", s2);
    s2.push('!');
    println!("s2:{}", s2);
}

// 使用+运算符拼接字符串
fn test1() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // s1所有权被移动了，不能继续使用，这里+运算符使用了add函数fn add(self, s: &str) -> String
    // &s2是&String类型，被强转成了&str，Rust使用了一个解引用强制转换（deref coercion）技术，
    // 把&s2变成了&s2[..]
    let s3 = s1 + &s2;
    // println!("s1:{}", s1);

    println!("s2:{}", s2);
    println!("s3:{}", s3);
}

// 使用format!宏拼接字符串
// format!宏不会获取任何参数的所有权
fn test2() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = format!("{}-{}", s1, s2);
    println!("s3:{}", s3);
    println!("s1:{}, s2={}", s1, s2);
}
