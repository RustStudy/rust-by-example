/*
Rust没有为基本数据类型提供隐式类型转换（coercion），但是可以使用as关键字来进行显示的类型转换（casting）

整数类型转换规则遵循C约定，除了C中未定义的一些情况。
所有的整数类型转换行为在Rust中都被定义好了。
*/

// Suppress all warnings from casts which overflow.
// 禁止所有类型转换溢出警告

#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // 错误！ 不能隐式转换
    // let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    // Explicit conversion
    // 显式转换
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // std::T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the result is the same as
    // first casting to the corresponding unsigned type then
    // taking the two's complement.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a i8 is : {}", 1000 as i8);
    // and the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);


}
