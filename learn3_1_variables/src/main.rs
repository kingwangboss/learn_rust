fn main() {
    // 创建不可变变量
    let x = 5;
    println!("The value of x is: {}", x);
    // 创建可变变量
    let mut y = 6;
    println!("The value of y is: {}", y);
    y = 10;
    println!("The value of y is: {}", y);
    // 创建常量
    const MAX_POINT: u32 = 1000000;
    println!("The value of MAX_POINT is: {}", MAX_POINT);

    //2.隐藏
    let y: f32 = 1.1;
    println!("y = {}",y);
}
