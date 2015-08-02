/*
fmt::Debug 看起来非常紧凑干净，所以它实现自定义的输出很方便。

通过手工实现fmt::Display，我们可以使用`{}`标识。
*/

// Import (via `use`) the `fmt` module to make it available.
// 通过use导入`fmt`模块
use std::fmt;

// Define a structure which `fmt::Display` will be implemented for. This is simply
// a tuple struct containing an `i32` bound to the name `Structure`.
// 定义一个元组结构体，供下面fmt::Display实现。
struct Structure(i32);

// In order to use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
// 为了使用`{}`, 一定要为该类型实现`fmt::Display`接口
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    // 该接口需要用`fmt`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.

        // 把输入流写入到第一个元素中：`f`。返回`fmt::Result`，指示操作成功或失败。
        // `write!`的语法跟`println!`非常相似
        write!(f, "{}", self.0)
    }
}

/*

fmt::Display比fmt::Debug更干净一些，但是这也展现了该std库的一个问题。
不明确的类型该如何显示呢？
例如，如果该std库要为所有的Vec<T>实现一个单独样式，那么应该实现下面哪种？

   Vec<path>: /:/etc:/home/username:/bin (split on :)
   Vec<number>: 1,2,3 (split on ,)

不，fmt::Display并没有为Vec<T>或其他泛型容器实现这样的样式。fmt::Debug必然是通用的。

*/

// use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        // 使用`self.number`来引用数据的位置
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

// Similarly, implement for Point2
impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2 { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // fmt::Display已经被fmt::Binary实现了，因此这里的用不上。std::fmt里有很多接口都需要自己的实现。
    // 细节可以去看std::fmt文档
    // println!("What does Point2D look like in binary: {:b}?", point);
}
