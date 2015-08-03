/*
类型声明可以赋予已经存在类型一个新的名字（别名，Alias）。类型一定要驼峰式命名，否则编译器会报警。
此规则对于一些基本类型不适用，比如usize，f32等。
*/


// `NanoSecond` is a new name for `u64`.
// `NanoSecond`是`u64`的新名字
type NanoSecond = u64;
type Inch = u64;

// Use an attribute to silence warning.
// 使用属性静默警告
#[allow(non_camel_case_types)]
type u64_t = u64;
// TODO ^ Try removing the attribute

// Use an attribute to silence warnings
#[allow(trivial_numeric_casts)]
fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    // 注意类型别名并没有提供任何额外的类型安全，因为别名不是新类型。
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}

/*
别名的主要用法是为了简化输入，比如IoResult<T>类型是Result<T, IoError>类型的别名。
*/
