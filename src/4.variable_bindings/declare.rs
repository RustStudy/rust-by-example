/*
也有可能先声明变量，然后再初始化它们。
*/

fn main() {
    // Declare a variable binding
    // 声明变量绑定
    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        // 初始化绑定
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // 错误！使用了未初始化的绑定
    // println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
