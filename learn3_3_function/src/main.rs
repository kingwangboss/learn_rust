fn main() {
    another_function1();
    another_function2(5);
    let x = plus_one(3);
    println!("x = {x}");
}

fn another_function1() {
    println!("无参无返回值函数！");
}

fn another_function2(x: i32) {
    println!("有参无返回值函数！参数x:{x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}