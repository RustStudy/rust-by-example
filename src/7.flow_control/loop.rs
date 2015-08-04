/*
Rust提供了loop关键字来表示无限循环。

break语句可以在任意时刻退出循环。而continue语句可以跳出本次循环并且开始新的循环。
*/

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            // 跳过本次迭代剩余的部分
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            // 退出循环
            break;
        }
    }
}
