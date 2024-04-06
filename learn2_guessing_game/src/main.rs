use rand::Rng;
use std::{cmp::Ordering, io};

// 程序随机生成1到100之间的整数。接着提示玩家猜测一个数并输入，然后指出猜测是大了还是小了，猜对了
// 打印祝贺信息并退出
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("猜测一个数，请输入你猜测的数：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("输入错误");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你输入的是：{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("猜中了");
                break;
            }
        }
    }
}
