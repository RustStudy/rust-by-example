/*
struct也能被解构
*/

fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    // destructure members of the struct
    // 解构结构体成员
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // you can destructure structs and rename the variables,
    // the order is not important
    // 你也可以解构结构体重命名变量
    // 顺序不重要
    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // and you can also ignore some variables:
    // 你也可以忽略变量
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // this will give an error: pattern does not mention field `x`
    // 模式如果没有匹配到`x`字段，就会报错
    // let Foo { y } = foo;
}
