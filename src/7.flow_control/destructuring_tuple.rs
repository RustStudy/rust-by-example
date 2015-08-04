/*
元组在match中可以被解构
*/

fn main() {
    let pair = (1, -2);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    // Match can be used to destructure a tuple
    // match用来解构元组
    match pair {
        // Destructure the second
        // 解构第二个
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
        // `_`意思是不要给变量绑定值
    }
}
