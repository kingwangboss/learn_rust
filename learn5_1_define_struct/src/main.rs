// 定义结构体
#[derive(Debug)]
struct User {
    active: bool,
    // 我们使用了自身拥有所有权的String类型而不是&str字符串的slice类型，因为我们需要结构体拥有它所有数据。
    username: String,
    email: String,
    sign_in_count: u64,
}

// 定义元组结构体
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// 定义类单元结构体(unit-like structs)，类似于元组的单元类型（unit type）（），
// 类单元结构体通常在某个类型上实现trait但不需要再类型中存储数据的时候用。
struct AlwaysEqual;


fn main() {
    let mut user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("email"), String::from("username"));

    // 使用user2中的值创建一个新的User实例
    let user3 = User {
        email: String::from("email3"),
        ..user2
    };
    println!("user3: {:#?}", user3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:?}, origin: {:?}", black, origin);
}

// 改变User实例字段
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}