use std::fmt::{Debug, Display};

// 定义trait
pub trait Summary {
    fn summarizy(&self) -> String;
}

// 默认实现
// pub trait Summary {
//     fn summarizy(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// 为类型实现trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarizy(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarizy(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as u pribably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarizy());
}

// trait作为参数
fn test1() {
    // impl Trait 语法
    // 我们可以定义一个函数 notify 来调用其参数 item 上的 summarize 方法，
    // 该参数是实现了 Summary trait 的某种类型。为此可以使用 impl Trait 语法。
    pub fn notify1(item: impl Summary) {
        println!("{}", item.summarizy());
    }

    // trait Bound语法
    pub fn notify2<T: Summary>(item1: T, item2: T) {
        println!("{}, {}", item1.summarizy(), item2.summarizy());
    }

    // 通过+指定多个trait bound
    // 如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法，
    // 那么 item 就需要同时实现两个不同的 trait：Display 和 Summary。这可以通过 + 语法实现
    pub fn notify3<T: Summary + Display>(item1: T, item2: T) {
        println!("{}, {}", item1.summarizy(), item2.summarizy());
    }

    // 通过where简化trait bound
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    fn some_function<T, U>(t: T, u: U) -> i32 
        where T: Display + Clone, U: Clone + Debug,
    {
        1
    }
}

// 返回实现了trait的类型
fn test2() {
    fn returns_summarizable() -> impl Summary {
        Tweet{
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

// 使用trait bound有条件地实现方法
// 通过使用带有 trait bound 的泛型参数的 impl 块，
// 可以有条件地只为那些实现了特定 trait 的类型实现方法。
// 例如，以下的类型 Pair<T> 总是实现了 new 方法，不过只有那些为 T 类型实现了 PartialOrd trait 
// （来允许比较） 和 Display trait （来启用打印）的 Pair<T> 才会实现 cmp_display 方法：
fn test3() {
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}