/*
Rust里的闭包(Closures，也叫做lambda，或匿名函数)是可以捕获封闭环境的函数。

特性：
- 使用||代替()包括输入变量
- 输入和返回的类型都能被推导
- 输入变量名称必须被指定
- 函数体界定符（{}）对于独立表达式是可选的。如果是多个表达式则必须加界定符
- 外层环境变量能被捕获
- 像函数一样调用闭包: call(var)
*/

fn main() {
    // Increment via closures and functions.
    fn  function(i: i32) -> i32 { i + 1 }

    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    // 闭包函数体的大括号`{}`在下面这种情况是可选的。
    // 匿名函数被指派给了变量。
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    // 调用函数和闭包
    println!("function: {}", function(i));
    println!("annotated closure: {}", closure_annotated(i));
    println!("inferred closure: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    // 无参数闭包返回`i32`类型
    // 返回类型是可推导的
    let one = || 1;
    println!("closure returning one: {}", one());

    // It is possible to capture variables from the enclosing
    // environment; something which is impossible with functions.
    // 可以从封闭的环境捕获变量，有些事是函数做不到的
    let professor_x = "Charles Xavier";

    // A closure which takes no argument, returning nothing, prints
    // a variable from the enclosing scope.
    // 无参闭包，无返回值，打印封闭的作用域中的变量
    let print = || println!("Professor X's name is: {}", professor_x);

    // Call the closure.
    // 调用闭包
    print();
}
