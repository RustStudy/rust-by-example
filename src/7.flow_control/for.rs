/*
for in用来构建循环迭代器和惰性值生成器（后面会有详细讲解）
创建迭代器最容易的方法是用范围（Range）标识。
a..b会从a（包含）到b（不包含）单步产生值

注：

两个点(..)有两种用法：
1. Range
2. 在match块（也包括let语句中的模式匹配）中被当作省略
*/

fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // `n` 会从1到100进行迭代
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
