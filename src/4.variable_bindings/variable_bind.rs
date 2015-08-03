/*
Rust通过静态类型来保证类型安全。变量绑定可以在声明的时候指定类型。
然而，大多数的情况下，编译器可以通过上下文推导出变量的类型。
*/

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    // 复制`an_integer`到`copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    // 对于没有用到的变量绑定，编译器会发出警告，这些警告可以给变量名加下划线前缀来消除

    let _unused_variable = 3u32;

    // let noisy_unused_variable = 2u32;
    let _noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
}
