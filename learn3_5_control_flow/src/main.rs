fn main() {
    let number = 3;
    // if表达式
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    let condition = true;
    // let语句使用if
    let number = if condition {
        5
    } else {
        6
    };
    println!("number:{number}");

    // Rust有三种循环 loop, while, for
    // loop {
    //     println!("loop");
    // }
    loop_label();
    loop_return();
    while_demo();
    collection_with_while();
    collection_with_for();
    range_rev();
}

// 循环标签
fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaing = 10;
        loop {
            println!("remaining = {}", remaing);
            if remaing == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaing -= 1;
        }
        count += 1;
    }
    println!("end count = {}", count);
}

// 循环返回
fn loop_return() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result);
}

// while循环
fn while_demo() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

// while 循环遍历数组
fn collection_with_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("value: {}", a[index]);
        index += 1;
    }
}

// for 循环遍历数组
fn collection_with_for() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("value: {}", element);
    }
}

fn range_rev() {
    for number in (1..=4).rev() {
        println!("{}!", number);
    }
}