/*
使用use声明就不需要手工指定scoping了
*/

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    // 显式的使用`use`，这样每个名字都可以直接使用了，不需要手工指定scoping
    use Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    // 自动使用`Work`里的每个名字
    use Work::*;

    // Equivalent to `Status::Poor`.
    // 等价于`Status::Poor`，因为使用了use，这里不需要再手工指定前面的scoping Status::了
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        // 注意这里未用scoping（Status::），因为上面显式的使用了use
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        // 注意，这里也没有使用scoping（Work::）
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}
