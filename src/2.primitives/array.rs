/*
数组是同一类型T存储在连续内存中的集合。
数组使用方括号创建，它的大小在编译时是已知的，大小也是类型签名的一部分 [T; size]。

切片（slice）跟数组类似，但是它的尺寸在编译时未知。切片对象包含两个元素，第一个是数据指针，另外一个是切片的长度。
切片被用来借用数组中一个片段，它的类型签名是 &[T]

*/


use std::mem;

// This function borrows a slice
// 该函数借用一个片段
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    // 把整个数组借用为切片
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Out of bound indexing yields a panic
    // 索引越界
    println!("{}", xs[5]);
}
