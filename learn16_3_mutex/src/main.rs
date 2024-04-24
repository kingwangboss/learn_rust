use std::{rc::Rc, sync::{Arc, Mutex}, thread, time::Duration};

// 互斥器一次只允许一个线程访问数据
/* 
互斥器（mutex）是 mutual exclusion 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据。
为了访问互斥器中的数据，线程首先需要通过获取互斥器的 锁（lock）来表明其希望访问数据。
锁是一个作为互斥器一部分的数据结构，它记录谁有数据的排他访问权。因此，
我们描述互斥器为通过锁系统保护（guarding）其数据。
互斥器以难以使用著称，因为你不得不记住：
在使用数据之前尝试获取锁。
处理完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁。
*/
fn main() {
    test4();
}

// Mutex<T>的API
fn test1() {
    // 使用关联函数new创建一个Mutex<T>
    let m = Mutex::new(5);
    {
        // 使用lock方法获取锁，以访问互斥器中的数据。这个调用会阻塞当前线程，知道我们拥有锁为止。
        // 如果另一个线程拥有锁，并且哪个线程panic了，则lock会调用失败，没人能够在获取锁，
        // 所以需要unwrap并在遇到这种情况时使线程panic.
        // 一旦获取了锁，就可以将返回值（在这里是num）视为一个其内部数据的可变引用了。
        // 类型系统确保了我们在使用 m 中的值之前获取锁：Mutex<i32> 并不是一个 i32，
        // 所以 必须 获取锁才能使用这个 i32 值。
        let mut num = m.lock().unwrap();
        // 正如你所怀疑的，Mutex<T> 是一个智能指针。更准确的说，lock 调用返回一个叫做 MutexGuard 的
        // 智能指针。这个智能指针实现了 Deref 来指向其内部数据；其也提供了一个 Drop 实现
        // 当 MutexGuard 离开作用域时自动释放锁。
        // 为此，我们不会冒忘记释放锁并阻塞互斥器为其它线程所用的风险，因为锁的释放是自动发生的。
        *num = 6;
    }

    // 丢弃了锁之后，可以打印出互斥器的值，并发现能够将其内部的 i32 改为 6。
    println!("m = {:?}", m);
}

// 在线程间共享Mutex<T>
fn test2() {
    // 创建一个counter变量来存放i32的Mutex<T>
    let counter = Mutex::new(0);
    let mut handles = vec![];

    // 创建10个线程，使用相同的闭包：每个都将调用lock方法来获取Mutex<T>上的锁，接着互斥器中
    // 的值加1，当一个线程结束执行，num或离开闭包作用域释放锁，这样另一个线程就可以获取它了。
    for _ in 0..10 {
        let handle = thread::spawn(move || {
        //    let mut num = counter.lock().unwrap();
        //    *num += 1; 
        });
        handles.push(handle);
    }

    for handle in handles {
        // 调用join方法来确保线程都会结束。
        handle.join().unwrap();
    }

    // println!("Result: {}", *counter.lock().unwrap());

    // 这是错误的，错误信息表明 counter 值在上一次循环中被移动了。
    // 所以 Rust 告诉我们不能将 counter 锁的所有权移动到多个线程中。
}

// 多线程和多所有权
fn test3() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            /* 
            编译器也告诉了我们原因 the trait bound `Send` is not satisfied。 
            Send：这是确保所使用的类型可以用于并发环境的 trait 之一。
            不幸的是，Rc<T> 并不能安全的在线程间共享。当 Rc<T> 管理引用计数时，
            它必须在每一个 clone 调用时增加计数，并在每一个克隆被丢弃时减少计数。
            Rc<T> 并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断。
            在计数出错时可能会导致诡异的 bug，比如可能会造成内存泄漏，或在使用结束之
            前就丢弃一个值。我们所需要的是一个完全类似 Rc<T>，又以一种线程安全的方式改
            变引用计数的类型。
             */
        //    let mut num = counter.lock().unwrap();
        //    *num += 1; 
        });
        handles.push(handle);
    }

    for handle in handles {
        // 调用join方法来确保线程都会结束。
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}

// 原子引用计数Arc<T>
/* 
 Arc<T> 正是 这么一个类似 Rc<T> 并可以安全的用于并发环境的类型。字母 “a” 代表 原子性（atomic），
 所以这是一个原子引用计数（atomically reference counted）类型。原子性是另一类这里还未涉及到的
 并发原语：请查看标准库中 std::sync::atomic 的文档来获取更多细节。其中的要点就是：原子性类型工作
 起来类似原始类型，不过可以安全的在线程间共享。
为什么不是所有的原始类型都是原子性的？
为什么不是所有标准库中的类型都默认使用 Arc<T> 实现？
原因在于线程安全带有性能惩罚，我们希望只在必要时才为此买单。
如果只是在单线程中对值进行操作，原子性提供的保证并无必要，代码可以因此运行的更快。
*/
fn test4() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

/* 
RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性
你可能注意到了，因为 counter 是不可变的，不过可以获取其内部值的可变引用；
这意味着 Mutex<T> 提供了内部可变性，就像 Cell 系列类型那样。
使用 RefCell<T> 可以改变 Rc<T> 中的内容那样，同样的可以使用 Mutex<T> 来改变 Arc<T> 中的内容。

另一个值得注意的细节是 Rust 不能避免使用 Mutex<T> 的全部逻辑错误。
使用 Rc<T> 就有造成引用循环的风险，这时两个 Rc<T> 值相互引用，造成内存泄漏。
同理，Mutex<T> 也有造成 死锁（deadlock） 的风险。这发生于当一个操作需要锁住两个资源
而两个线程各持一个锁，这会造成它们永远相互等待。
*/
fn test5() {
    // 这个例子有两个线程handle1和handle2,有两个互斥量mutex1和mutex2,
    // handle1线程先获取mutex1的锁定，然后尝试获取mutex2的锁定。
    // 同时handle2线程先获取mutex2的锁定，然后尝试获取mutex1的锁定。
    // 两个线程都在尝试获取对方已持有的锁，导致死锁。
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));

    let mutex1_clone = Arc::clone(&mutex1);
    let mutex2_clone = Arc::clone(&mutex2);

    let handle1 = thread::spawn(move || {
        let lock1 = mutex1_clone.lock().unwrap();
        println!("thread1 acquired lock on mutex1");

        thread::sleep(Duration::from_secs(1));

        let lock2 = mutex2_clone.lock().unwrap();
        println!("thread1 acquired lock on mutex2");

        println!("thread1 processing data: {}", *lock1 + *lock2);
    });

    let handle2 = thread::spawn(move || {
        let lock2 = mutex2.lock().unwrap();
        println!("thread2 acquired lock on mutex2");
        thread::sleep(Duration::from_secs(1));
        let lock1 = mutex1.lock().unwrap();
        println!("thread2 acquired lock on mutex1");
        println!("thread2 processing data: {}", *lock1 + *lock2);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}