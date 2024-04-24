use std::{thread, time::Duration};

fn main() {
    test2();
}

// 使用spawn创建新线程,主线程提前结束了，所有这里的子线程可能不能够执行完。
fn test1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// 使用join等待所有线程结束
fn test2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 如果在这个位置使用join(),主线程会等待直到新线程执行完毕后才开始执行主线程的工作。
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // JoinHandle是一个拥有所有权的值，当对其调用join方法时，它会等待其线程结束。
    handle.join().unwrap();
}

