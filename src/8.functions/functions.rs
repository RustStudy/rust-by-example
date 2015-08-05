/*
函数是用fn关键字声明的。它的参数是需要标注类型的，就像变量。
如果函数有返回值，那么返回类型一定要指派在箭头后面 ->

函数中最后的表达式会被用作返回值，或者使用returen关键字来返回一个值，即便是在loop或if中。

*/

// Unlike C/C++, there's no restriction on the order of function definitions
// 不像C/C++，没有限制函数定义的顺序
fn main() {
    // We can use this function here, and define it somewhere later
    // 我们可以在这里使用这个函数，而它被定义在后面的某些地方
    fizzbuzz_to(100);
}

// Function that returns a boolean value
// 返回布尔值的函数
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    // 满足条件就会提前return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    // 这是一个表达式，这里不需要return关键字
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}
