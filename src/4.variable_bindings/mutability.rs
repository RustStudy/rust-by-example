/*

变量绑定默认是不可变的，但是可以使用mut重新修改

*/

fn main() {
    let _immutable_binding = 1;
    // let mut mutable_binding = 1;
    let mut immutable_binding = 1;
    // println!("Before mutation: {}", mutable_binding);
    println!("Before mutation: {}", immutable_binding);

    // Ok
    // mutable_binding += 1;
    _immutable_binding += 1;

    // println!("After mutation: {}", mutable_binding);
    println!("After mutation: {}", immutable_binding);

    // Error!
    // _immutable_binding += 1;
    // FIXME ^ Comment out this line
}
