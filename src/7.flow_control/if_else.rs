/*
if-else分支控制结构和其他语言相似。但是跟大部分语言不同的是，布尔条件不需要括号，每个条件后面必须跟随block。
if-else条件是表达式，并且所有的分支都必须返回相同的类型。
*/

fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            // 该表达式返回`i32`类型
            10 * n
        } else {
            println!(", and is a big number, reduce by two");

            // This expression must return an `i32` as well.
            // 该表达式也必须返回`i32`类型
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
            // TODO ^ 尽量禁止表达式使用分号
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.
    // 不要忘记在这里添加分号，所有的`let`绑定都需要分号。

    println!("{} -> {}", n, big_n);
}
