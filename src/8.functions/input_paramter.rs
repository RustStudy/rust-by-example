/*
在函数里，闭包必须标注类型，使用方式如下：

 - Fn：捕获引用（&T）
 - FnMut: 捕获可变引用（&mut T）
 - FnOnce：捕获值（T）

即使标注类型，它们也是很灵活的：FnOnce指派参数的闭包可能捕获T或&mut T或&T（如果move可以，borrow应该也可以）
反之则不行：如果参数是Fn，那么其他级别的都不允许。因此规则是：

  - 任何标注的参数严禁捕获自身和以上

*/

// A function which takes a closure as an argument and calls
// it. The closure takes no input and returns nothing.
// 函数用闭包作为参数并调用它。 该闭包无输入参数或返回值。
fn apply<F>(f: F) where
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f()
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    let greeting = "hello";
    // A non-copy type.
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        // `greeting`是一个引用，需要`Fn`
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        // `farewell`捕获的是可变引用，需要`FnMut`
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        // 手工调用drop强制`farewell`为捕获值。 需要`FnOnce`
        drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
