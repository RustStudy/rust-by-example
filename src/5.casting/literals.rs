/*
数字字面量可以通过增加类型后缀来标注类型，usize和isize分别使用usize和isize为后缀。
无后缀的数字字面量的类型依赖于它们如何被用。如果没有约束存在，编译器将会为整数使用i32，浮点数使用f64.
*/
use std::mem;

fn main() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    // 无后缀字面量，它们的类型依赖于如何使用
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    // `size_of_val`返回变量的字节大小
    println!("size of `x` in bytes: {}", mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

/*
   fun(&foo) 用于给函数按引用传入参数，与之相对的是按值传参（fun(foo)）,更多的细节参考borrowing.
   std::mem::size_of_val是一个函数，但是这里使用了全路径调用。代码可以被分隔成独立的逻辑单元：module（模块）。
   在本例中，size_of_val函数被定义在mem模块中，mem模块被定义在std包（crate）中。

*/
