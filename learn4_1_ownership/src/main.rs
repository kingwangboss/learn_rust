fn main() {
    /*
       所有权规则
       1.Rust中每一个值都有一个被称为其所有者的变量。
       2.值在任意时刻有且只有一个所有者。
       3.当所有者（变量）离开作用域，这个值将被舍弃。
    */
    owner1();
    owner2();

    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效
    let x = 4; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn owner1() {
    let x = 5;
    let y = x;
    // int类型变量内存分配在栈上，因此将5绑定到x， x绑定到y, 都把5放到了栈上。x, y 都能使用。
    // int,bool,float,char，部分tuple都有copy trait特殊标注，
    // 如果一个类型实现了copy trait那么一个旧的变量赋值给其他变量后仍然可用。
    // Rust不允许自身或者其他任何部分实现了Drop trait的类型使用Copy trait.
    println!("x: {}", x);
    println!("y: {}", y);

    let s1 = String::from("hello");
    let s2 = s1;
    // String创建的数据在堆上，s1是指向堆数据的指针，s2 = s1这个操作是移动所有权，此时s2指向堆数据，
    // s1因所有权丢失被释放，后面不能被使用。
    // println!("s1: {}", s1);
    println!("s2: {}", s2);
}

fn owner2() {
    // 如果想要深度复制String中堆上的数据可以使用clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}