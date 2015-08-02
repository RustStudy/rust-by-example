/*
enum也能被用作C-like的enum

*/

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// enum with implicit discriminator (starts at 0)
#[derive(Debug)]
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers.
    // `enum`可以转换为整数
    println!("zero is {}", Number::Zero as i32);
    // 直接输出
    println!("zero is {:?}", Number::Zero);
    println!("one is {}", Number::One as i32);
    println!("Two is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
