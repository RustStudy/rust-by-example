/*
Rust主要通过下面两个关键字来自定义类型：

struct 定义一个结构体
enum 定义个枚举类

常量可以通过const和static关键字来创建

3.1 结构体

使用struct能创建三种类型的结构体：
 - 元组结构体，基本的命名结构体
 - 经典的C结构体
 - 单元结构体，无字段，可用于泛型

*/

// A unit struct
// 单元结构体
struct Nil;

// A tuple struct
// 元组结构体
struct Pair(i32, f64);

// A struct with two fields
// 有两个字段的结构体（类C结构体）
struct Point {
    x: f64,
    y: f64,
}

// Structs can be reused as fields of another struct
// 结构体可以用作另一个结构体中的字段
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    // 访问point的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    // 使用let绑定解构point
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Destructure a tuple struct
    // 解构元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
