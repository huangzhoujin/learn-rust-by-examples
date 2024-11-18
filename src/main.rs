/*
 Rust 专门为解决这个问题提供了 ? 操作符。
 */
use std::fmt; // 导入 `fmt` 模块。

// 定义一个包含单个 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 { write!(f, ", ")?; }
            /*
              更改程序使 vector 里面每个元素的下标也能够打印出来。新的结果如下：
              [0: 1, 1: 2, 2: 3]
            */
            write!(f, "{} : {}", count, v)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    let v2 = List(vec![]);
    println!("{}", v2);
}