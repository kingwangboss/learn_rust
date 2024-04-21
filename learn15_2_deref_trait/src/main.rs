use std::ops::Deref;

fn main() {
    test1();
    test2();
    test3();
    test4();
}

//实现 Deref trait 允许我们重载 解引用运算符（dereference operator）*（与乘法运算符或通配符相区别）。
//通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。
fn test1() {
    // 通过解引用运算符追踪指针的值
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 像引用一样使用Box<T>
fn test2() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 自定义智能指针MyBox<T>
fn test3() {

    // 这里定义了一个结构体 MyBox 并声明了一个泛型参数 T，因为我们希望其可以存放任何类型的值。
    // MyBox 是一个包含 T 类型元素的元组结构体。MyBox::new 函数获取一个 T 类型的参数并返回一
    // 个存放传入值的 MyBox 实例。
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    // 通过实现deref trait将某类型像引用一样处理
    impl<T> Deref for MyBox<T> {
        // 定义了用于此 trait 的关联类型。
        type Target = T;
        // deref 方法体中写入了 &self.0，这样 deref 返回了我希望通过 * 运算符访问的值的引用。
        fn deref(&self) -> &Self::Target {
            &self.0
        }
        // 没有 Deref trait 的话，编译器只会把 & 引用类型解引用。
        // deref 方法向编译器提供了一种能力：能够获取任何实现了 Deref trait 的类型的值，
        // 并且可以通过调用这个类型的 deref 方法来获取一个解引用方法已知的 & 引用。
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // *y 底层运行的就是 *(y.deref())
    assert_eq!(5, *(y.deref()));
}

// 函数和方法的隐式解引用强制转换
fn test4() {
    /* 
    解引用强制转换（deref coercions）是 Rust 在函数或方法传参上的一种便利。
    解引用强制转换只能工作在实现了 Deref trait 的类型上。解引用强制转换将一种
    类型（A）隐式转换为另外一种类型（B）的引用，因为 A 类型实现了 Deref trait，
    并且其关联类型是 B 类型。比如，解引用强制转换可以将 &String 转换为 &str，
    因为类型 String 实现了 Deref trait 并且其关联类型是 str。 */
    fn hello(name: &str) {
        println!("Hello, {}", name);
    }

    let m = Box::new(String::from("Rust"));
    // 使用&m调用hello函数，因为Box<T>实现了Deref trait,Rust就可以通过deref调用
    // 将&Box<String>变为&String.标准库中提供了String上的Deref实现，其会返回slice,
    // 这可以在Deref的API文档中看到：
    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl ops::Deref for String {
    //     type Target = str;
    //     #[inline]
    //     fn deref(&self) -> &str {
    //         unsafe { str::from_utf8_unchecked(&self.vec) }
    //     }
    // }
    // Rust再次调用deref将&String变为&str.
    hello(&m);
    // (*m) 将 Box<String> 解引用为 String。接着 & 和 [..] 获取了
    // 整个 String 的字符串 slice 来匹配 hello 的签名。
    // 没有解引用强制转换所有这些符号混在一起将更难以读写和理解。
    // 解引用强制转换使得 Rust 自动的帮我们处理这些转换。
    hello(&(*m)[..]);
    // 当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意
    // 多次 Deref::deref 调用以获得匹配参数的类型。这些解析都发生在编译时，
    // 所以利用解引用强制转换并没有运行时损耗！
}

// 当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用
// 任意多次 Deref::deref 调用以获得匹配参数的类型。这些解析都发生在编译时，
// 所以利用解引用强制转换并没有运行时损耗！
fn test5() {
    /* 
    类似于使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载
    可变引用的 * 运算符。
    Rust 在发现类型和 trait 的实现满足以下三种情况时会进行解引用强制转换：
    当 T: Deref<Target=U> ：从 &T 到 &U。
    当 T: DerefMut<Target=U> ：从 &mut T 到 &mut U。
    当 T: Deref<Target=U> ：从 &mut T 到 &U。
    前两种情况除了可变性之外是相同的：第一种情况表明如果有一个 &T，而 T 实现了返回 U 类型的 Deref，
    则可以直接得到 &U。第二种情况表明对于可变引用也有着相同的行为。

    第三种情况有些微妙：Rust 也会将可变引用强转为不可变引用，但是反之是 不可能 的，
    因为不可变引用永远也不能强转为可变引用。因为根据借用规则，如果有一个可变引用，
    其必须是这些数据的唯一引用（否则程序将无法编译）。将一个可变引用转换为不可变引用永远也
    不会打破借用规则。将不可变引用转换为可变引用则需要数据只能有一个不可变引用，而借用规则
    无法保证这一点。因此，Rust 无法假设将不可变引用转换为可变引用是可能的。 */
}