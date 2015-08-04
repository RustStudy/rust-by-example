/*
指针跟C语言还是有所区别

使用*来解引用
使用&, ref, 和 ref mut来解构值

*/

fn main() {
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    // 指派类似为`i32`的引用。这里的`&`表示被指派的是引用。
    let reference = &4;

    match reference {
        // If `reference`s is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        // 如果`引用`的模式匹配对应于`&val`, 这样产生的比较就类似于这样：
        // `&i32`
        // `&val`
        // 如果把匹配的`&`丢掉，那么剩下的`i32`就指派给了`val`

        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    // 避免`&`， 在匹配之前解构引用
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    //  右侧不加`&`就不是引用
    let _not_a_reference = 3;

    // Rust provides `ref` for exacty this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    // Rust提供了`ref`来修改已经创建的元素。
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    // 因此，定义两个不是引用的值，可以通过`ref`和`ref mut`来取到引用
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    // 使用`ref`来创建引用
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    // `ref mut`用法相似
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            // 得到的是引用，计算前需要解构引用
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
