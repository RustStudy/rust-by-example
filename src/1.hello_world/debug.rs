/*
想用std::fmt的所有类型接口（trait）都需要实现打印功能。
仅std库中的类型自动实现了该功能，而其他类型必须手工实现。

fmt::Debug接口使这一切变简单了。所有类型都可以通过derive（自动创建）fmt::Debug实现。
fmt::Display必须手工实现。
*/

// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
// `derive`属性用`fmt::Debug`自动创建了使此结构体可打印的实现
#[derive(Debug)]
struct DebugPrintable(i32);

//

// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    // `{:?}` 和 `{}`相似.
    // 具体细节参考API文档：https://doc.rust-lang.org/stable/std/fmt/index.html
    // nothing ⇒ Display
    // ? ⇒ Debug
    // o ⇒ Octal
    // x ⇒ LowerHex
    // X ⇒ UpperHex
    // p ⇒ Pointer
    // b ⇒ Binary
    // e ⇒ LowerExp
    // E ⇒ UpperExp
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // 只想显示7
    // 解构元组结构体
    let Deep(Structure(i)) = Deep(Structure(7));
    println!("Now {:?} will print!", i);

}
