/* 大部分错误并没有严重到需要程序完全停止执行。有时，一个函数会因为一个容易理解并做出反应的原因失败。
例如，如果因为打开一个并不存在的文件而失败，此时我们可能想要创建这个文件，而不是终止进程。 */

use std::{error::Error, fs::{self, File}, io::{self, ErrorKind, Read}};

fn main() {

}

fn test1() {
    let f = File::open("hello.txt");

    let f = match f {
        // 注意与 Option 枚举一样，Result 枚举和其成员也被导入到了 prelude 中，
        // 所以就不需要在 match 分支中的 Ok 和 Err 之前指定 Result::。
        Ok(file) => file,
        Err(err) => {
            println!("打开文件失败，创建文件！");
            match err.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(e) => panic!("文件创建失败：{}", e),
                },
                other => panic!("其他错误：{}", other),
            };
            panic!("打开文件失败：{}", err);
        }
    };
}

fn test2() {
    /* match 能够胜任它的工作，不过它可能有点冗长并且不总是能很好地表明其意图。
    Result<T, E> 类型定义了很多辅助方法来处理各种情况。其中之一叫做 unwrap，
    它的实现就类似于示例 9-4 中的 match 语句。如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
    如果 Result 是成员 Err，unwrap 会为我们调用 panic!。 */
    let f = File::open("hello.txt").unwrap();
}

fn test3() {
    /* 还有另一个类似于 unwrap 的方法它还允许我们选择 panic! 的错误信息：expect。使用 expect 而不
    是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。 */
    let f1 = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 传播错误
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return  Err(e),
    };

    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// 传播错误的简写：? 运算符
/* match 表达式与问号运算符所做的有一点不同：? 运算符所使用的错误值被传递给了 from 函数，
它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。当 ? 运算符调用 from 函数
时，收到的错误类型被转换为由当前函数返回类型所指定的错误类型。这在当函数返回单个错误类型来代表所有可
能失败的方式时很有用，即使其可能会因很多种原因失败。只要每一个错误类型都实现了 from 函数来定义如何
将自身转换为返回的错误类型，? 运算符会自动处理这些转换。 */
fn read_username_from_file1() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

/* 将文件读取到一个字符串是相当常见的操作，所以 Rust 提供了名为 fs::read_to_string 的函数，它会打开文件、新建一个 String、读取文件的内容，并将内容放入 String，接着返回它。当然，这样做就没有展示所有这些错误处理的机会了，所以我们最初就选择了艰苦的道路。 */
fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ? 运算符可被用于返回 Result 的函数
fn test4() -> Result<(), Box<dyn Error>> {
    // ? 运算符可被用于返回值类型为 Result 的函数,因为他被定义为 match 表达式有着完全相同的工作方式。match 的 return Err(e) 部分要求返回值类型是 Result，所以函数的返回值必须是 Result 才能与这个 return 相兼容。
    // let f = File::open("hello.txt")?;
    let f = File::open("hello.txt")?;
    Ok(())
}