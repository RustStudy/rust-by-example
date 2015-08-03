/*
Rust程序（大部分）由一系列的语句组成：

fn main() {
   // statement
   // statement
   // statement
}

Rust中有几种语句。最常见的两种是声明变量绑定和使用了分号（;）的表达式（expression）

fn main() {
    // variable binding
    // 变量绑定
    let x = 5;

    // expression;
    // 表达式;
    x;
    x + 1;
    15;
}

块（Block）也是表达式，所以它们能被用作r-value来指派。在块中的最后一个表达式将会指派给l-value。
然而，如果是在块中的最后一个表达式以分号结尾，则会返回()

*/
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        // 该表达式将会指派给`y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        // 分号禁止该表达式，并且返回`()`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
