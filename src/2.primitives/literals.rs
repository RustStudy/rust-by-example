/*
整数1， 浮点数1.2， 字符'a'，字符串"abc"，布尔值 true和单元类型()，都是字面量
整数也可以使用这些前缀分别表示十六进制、八进制或二进制：0x, 0o 或 0b
数字字面量可以使用下划线来提升可读性。比如1_000跟1000相同，0.000_001跟0.000001相同
我们需要告诉编译器我们使用字面量的类型。我们使用u32后缀来表示无符号32位整数，使用i32后缀来表示符号型32位整数。

可用的运算符和优先级跟其他类C语言一致。
*/


fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // 这种会报错：溢出
    // error: attempted to sub with overflow
    // println!("1 + 2 = {}", 1u32 - 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
    // 跟上面error类似

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:02b}", 0b0011u32 & 0b0101);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
