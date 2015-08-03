/*
变量绑定是有作用域范围（scope）的，是由大括号{}来约束的区域。
*/

fn main() {
    // This binding lives in the main function
    // 该绑定存活于main函数中
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    // 这个block，比main函数的作用域范围更小
    {
        // This binding only exists in this block
        // 该绑定仅存在于此block中
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // This binding *shadows* the outer one
        // 该变量把外层同名变量给屏蔽了
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // `short_lived_binding`不存在于此作用域，
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);

    // This binding also *shadows* the previous binding
    // 该绑定也会`屏蔽`前一个绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}
