/*
类型推断引擎是相当聪明的。

它做的比在初始化期间专注于r-value类型的要更多。
r-value(https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue)

 l-value：变量在内存中的位置。通过它能够找到内存中存放的变量（location value）
 r-value：存放在lvalue对应的内存中的值（register value）
 值传递传的是r-value，引用传递传的是l-value

 它也专注于变量被使用后来推导该变量的类型。

 下面是类型推导的高级用法：

*/

fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    // 因为类型标注，编译器知道`elem`是u8类型。
    let elem = 5u8;

    // Create an empty vector (a growable array).
    // 创建一个空vector（可增长数组）
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).
    // 此时编译器不知道`vec`的精确类型，它只知道它是某些东西的载体（`Vec<_>`）

    // Insert `elem` in the vector.
    // 在vector中插入`elem`
    vec.push(elem);

    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // 现在编译器知道`vec`是`u8`类型的载体了（`Vec<u8>`）
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}
