/*
闭包可以灵活的捕捉变量： move和borrow
*/

fn main() {
    let color = "green";

    // A closure to print `color` which immediately borrows (`&`)
    // `color` and stores the borrow and closure in the `print`
    // variable. It will remain borrowed until `print` goes out of
    // scope. `println!` only requires `by reference` so it doesn't
    // impose anything more restrictive.
    // print闭包在`print`变量中存储借用和闭包，打印`color`的立即值借用（`&`）
    // 借用一直保留在`print`出了作用域。`println!`仅需要`引用`，
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();
    print();

    let mut count = 0;

    // A closure to increment `count` could take either `&mut count`
    // or `count` but `&mut count` is less restrictive so it takes
    // that. Immediately borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside.
    // Thus, calling the closure mutates the closure which requires
    // a `mut`.
    // 闭包中存储的是可变的引用`&mut count`，需要`mut`闭包
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure.
    inc();
    inc();

    // 可变变量只能借用一次
    // let reborrow = &mut count;
    // ^ TODO: try uncommenting this line.

    // A non-copy type.
    // 非拷贝类型，堆
    let movable = Box::new(3);

    // `drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    // `drop`需要`T`，所以这里必须有值。拷贝类型会把值拷贝到闭包中，而留下原始的值未动。
    // 非拷贝类似必须移动（move），所以`moveable`立即值移动到了闭包中
    let consume = || {
        println!("`movable`: {:?}", movable);
        drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    // `consume` 仅能调用一次
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.
}
