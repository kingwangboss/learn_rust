// match是Rust极为强大的控制流运算符，模式匹配，模式可以由字面量、变量、通配符和许多其他内容构成
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
// match和if非常大的区别：对于if，表达式必须返回一个布尔值，而match可以返回任何类型。
// match分支有两个部分：一个是模式，一个是代码块
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("匹配Penny，执行代码块。");
            1
        },
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter => 4,
    }
}

// 绑定值的模式
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin1 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents1(coin: Coin1) -> u8 {
    match coin {
        Coin1::Penny => 1,
        Coin1::Nickel => 2,
        Coin1::Dime => 3,
        Coin1::Quarter(state) => {
            println!("state: {:?}", state);
            4
        }
    }
}
// 匹配Option<T>
fn test1() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            // Rust匹配是穷尽的：必须穷举到最后可能性来使代码有效，不匹配None代码编译报错。
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}, six:{:?}, none:{:?}", five, six, none);
}
// 通配符和_占位符
fn test2() {
    let dice_roll = 9;
    fn add_fancy_hat() {
        
    }
    fn remove_fancy_hat() {
        
    }
    fn move_play(num_spaces: u8) {
        
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 在这种通配模式匹配下满足了match必须被穷尽的要求，我们必须把通配分支放最后，因为模式匹配是顺序的。
        // 模式other是一个变量，在other分支的代码可以通过这个变量传递给代码块使用。
        other =>move_play(other),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 如果我们不想用通配模式获取变量，可以使用_，这是一个特殊模式，可以匹配任意值而不绑定到该值。
        // 如果匹配分支后面没有需要执行的代码块还可以用空元组 
        _ => (),
    }
}
fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents1(Coin1::Quarter(UsState::Alabama));
    test1();
}
