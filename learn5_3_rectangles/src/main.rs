/* 方法与函数类似：它们使用fn关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法
时会执行的代码。不过方法与函数不同的，因为它们在结构体的上下文中被定义（或是枚举或trait对象的
上下文）并且它们的第一个参数总是self，它代表调用该方法的结构体实例。 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 这个impl块中的所有内容都与Rectangle类型相关联，&self代替了rectangle:&Rectangle,
    // &self实际上是self:&Self的缩写。
    fn area(&self) -> u32 {
        // 实际上是(*self).width * (*self).height;Rust有个叫自动引用和解引用的功能。
        self.width * self.height
    }

    /// 是否能包含另一个长方形的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 我们定义不以self为第一参数的函数叫关联函数（因此不是方法）,它们不作用于一个结构体实例
    /// 定义一个创建正方形的关联函数 
    fn squate(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // 实例调用方法实际上是(&rect1).area();Rust有个叫自动引用和解引用的功能。
    println!("面积:{}", rect1.area());

    println!("rect1是否能包含rect2: {}", rect1.can_hold(&rect2));
    println!("rect1是否能包含rect3: {}", rect1.can_hold(&rect3));

    // 关联函数调用
    let sq = Rectangle::squate(3);
    println!("sq:{:#?}", sq);
}
