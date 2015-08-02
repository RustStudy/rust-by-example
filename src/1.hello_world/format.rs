/*
定义于 std::fmt中的一系列宏（macros）定义可以处理打印：
1. format!  ： 格式化String文本
2. print!  ： 跟format!类似，但是在控制台格式化文本
3. println! : 跟print!一样，但是会输出换行符
*/

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    // 一般情况下，`{}`会自动替换掉任何参数。这些参数都会变成字符串。
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is,
    // with a suffix.
    // 没有后缀，31是i32类型。你可以使用后缀改变31的类型。
    println!("{} days", 31i64);
    // There are various optional patterns this works with. Positional
    // arguments can be used.
    // 当有多个选项匹配的时候，可以使用位置参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    // 也可以用命名参数
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");

    // Special formatting can be specified after a `:`.
    // 可以在`:`后面指定特殊的格式

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // It will even check to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure which contains an `i32`. Name it `Structure`.
    // 创建了一个i32结构体
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // 对于结构体需要更多的编译处理
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}
