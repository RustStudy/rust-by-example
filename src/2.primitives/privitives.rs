/*
Rust提供种类繁多的基本类型。包括：

符号整型： i8, i16, i32, i64 和 isize（指针大小）
无符号整型： u8, u16, u32, u64 和 usize（指针大小）
浮点数： f32, f64
字符（char）： Unicode标量值，像'a', 'α' 和 '∞' (每个4字节)
布尔值（bool）：true和false
单元类型(unit type)： 仅有的值就是一对括号()
数组(array):  [1, 2, 3]
元组(tuple): (1, true)

变量总是需要标注类型。 通过后缀或默认都可以标注。整数默认为i32， 浮点数默认为f64。
*/

fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 正常标注类型
    let an_integer   = 5i32; // 后缀标注

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    let mut mutable = 12; // 可变 `i32`类型.

    // Error! The type of a variable can't be changed
    // 这样会出错，变量的类型是不能被改变的。
    // mutable = true;

}
