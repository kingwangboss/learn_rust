// 计算长方形面积

// println!宏能处理很多类型的格式，基本类型默认实现了Display,结构体没有提供一个Display实现。
// 加入{:?}指示符告诉println!我们想要使用叫做Debug的输出格式，Debug是一个trait,
// 它允许我们打印结构体。我们需要加入外部属性#[derive(Debug)]
#[derive(Debug)]
// 定义长方形结构体
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("面积:{}", area(&rect1));
    println!("rect1:{:#?}", rect1);
    // 另一种使用Debug格式打印数值的方法是dbg!宏，dbg!宏接收一个表达式的所有权，打印出
    // 代码中调用dbg!宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。
    // dbg!(rect1);
    // 我们不希望dbg!拥有rect1的所有权，所有我们应该传入一个引用。
    dbg!(&rect1);
}
