/*
enum关键字允许创建不同类型的变量。对结构体有效的，对enum同样有效。

*/

// An attribute to hide warnings for unused code.
// 此属性可以隐藏无用代码的警告
#![allow(dead_code)]

// Create an `enum` to classify someone. Note how both names
// and type information together specify the variant:
// `Skinny != Fat` and `Height(i32) != Weight(i32)`. Each
// is different and independent.
enum Person {
    // An `enum` may either be `unit-like`,
    //  一个像单元结构体的枚举元素
    Skinny,
    Fat,
    // like tuple structs,
    // 像元组结构体
    Height(i32),
    Weight(i32),
    // or like structures.
    // 或者像结构体
    Info { name: String, height: i32 }
}

// A function which takes a `Person` enum as an argument and
// returns nothing.
// 把`Person`枚举类型当作此函数的参数，没有返回值
fn inspect(p: Person) {
    // Usage of an `enum` must cover all cases (irrefutable)
    // so a `match` is used to branch over it.
    // enum的用法必须涵盖所有情况
    match p {
        Person::Skinny    => println!("Is skinny!"),
        Person::Fat       => println!("Is fat!"),
        // Destructure `i` from inside the `enum`.
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // Destructure `Info` into `name` and `height`.
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person = Person::Height(18);
    // `to_owned()` creates an owned `String` from a string slice.
    // `to_owned()` 从字面量那创建一份自己的`String`  参考： https://doc.rust-lang.org/std/borrow/trait.ToOwned.html
    let dave   = Person::Info { name: "Dave".to_owned(), height: 72 };
    // ^ TODO: Try changing these to a different variants.
    let foo = Person::Fat;

    inspect(person);
    inspect(dave);
    inspect(foo);
}
