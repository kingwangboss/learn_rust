use std::{sync::mpsc, thread, time::Duration};

fn main() {
    test4();
}

// 使用消息传递在线程间传送数据
fn test1() {
    // 使用mpsc::channel函数创建一个新的通道，mpsc是多个生产者，
    // 单个消费者(multiple producer,single consumer)的缩写，
    // 函数返回一个元组，tx是发送者(transmitter)，rx是接收者(receiver)的缩写
    let (tx, rx) = mpsc::channel();

    // move将tx移动到闭包中，这样新建线程就拥有tx了。
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // 通道的接收端有两个有用的方法：recv和try_recv，recv会阻塞主线程执行知道从通道中接收一个值，
    // try_recv不会阻塞，它立即返回一个Result<T, E>: Ok包含可用信息，而Err代表此时没有任何消息。
    // 如果线程在等待消息的过程还有其他工作时使用try_recv。
    // 这里是因为主线程除了等待消息之外没有任何其他工作，所以阻塞主线程是合适的。
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// 通道与所有权转移
fn test2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // 这里尝试在新建线程中的通道中发送完val值之后再使用它，Rust是不允许这样做的。
        // 一旦值发送到另一个线程后，那个线程可能会在我们再次使用它之前就将其丢弃或修改，
        // 如果允许这样做会导致这个值不一致或不存在，导致错误或意外的结果。
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// 发送多个值并观察接收者的等待
fn test3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    // 在主线程不再显式调用recv函数，而是将rx当作一个迭代器。当通道关闭时，迭代器也将结束。
    // 因为主线程中for循环里没有暂停或等待的代码，所以可以说主线程是在等待从新建线程中接收值。
    for received in rx {
        println!("Got: {}", received);
    }
}

// 通过克隆发送者来创建多个生产者
fn test4() {
    let (tx, rx) = mpsc::channel();

    // 我们对通道的发送端调用了clone方法，这会给我们一个可以传递给第一个新建线程的发送端句柄。
    // 我们会将原始的通道发送端传递给第二个新建线程，这样就有两个线程，每个线程将向通道的接收端发送不同的消息。
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}