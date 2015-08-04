/*
在嵌套循环的情况下，有时候会需要break或continue到外层循环，循环必须要使用标签（'label）标注，
并且这些标签必须传入break或continue语句中。
*/


#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            // 这个break仅退出内部循环,这样用的话，很容易死循环
            // break;

            // This breaks the outer loop
            // 这个break会退出外部循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
