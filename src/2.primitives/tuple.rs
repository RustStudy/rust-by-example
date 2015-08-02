/*
元组（Tuple）是不同类型值的集合。
元组使用圆括号和携带类型签名的值组成。(T1, T2, ...)其中的T1，T2是元组成员的类型。
函数可以使用元组返回多个值，元组能保存任意数量的值。

*/

// Tuples can be used as function arguments and as return values
// 元组能用作函数参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    // `let`能用于为变量绑定元组的成员
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn main() {
    // A tuple with a bunch of different types
    // 包含不同类型值的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Values can be extracted from the tuple using tuple indexing
    // 可以使用元组的索引来取值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    // 元组也可用作元组成员
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    // 元组可打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    // 创建一个元素的元组， 使用逗号来区分被圆括号包含的字面量
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    // 解构元组创建绑定
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}
