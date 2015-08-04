/*
Rust通过match关键字来提供模式匹配， 用法相当于C语言中的switch，或者Ruby中的case语句
*/

fn main() {
    let number = 5;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        // 匹配单个值
        1 => println!("One!"),
        // Match several values
        // 匹配多值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        // 匹配包含的范围
        13...19 => println!("A teen"),
        // Handle the rest of cases
        // 处理其他情况
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    // 也能匹配表达式
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        // match里必须匹配所有可能的值
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}
