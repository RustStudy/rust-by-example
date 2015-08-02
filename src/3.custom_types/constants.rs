/*
Rust有两种不同类型的常量（constant），可以在任何作用域中声明，包括全局作用域。
两者都需要显式的声明类型：

   const: 不能改变的值 (常见).
   static: 用'static生存期可能会改变的变量

一个特殊的情况是"字符串"字面量。它可以直接分配给一个静态变量，而无需修改，
因为它的类型签名: &'static str 需要'static生存期。其他所有的引用类型必须专门指定以便它们可以执行'static生存期。
*/


// Globals are declared outside all other scopes.
// 全局声明
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    // 在函数中访问常量
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    // 在main线程中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // 不能修改`const`常量
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}
