/*
while let 跟if let类似，可以简化复杂的match序列，比如：

// Make `optional` of type `Option<i32>`
let mut optional = Some(0);

// Repeatedly try this test.
loop {
    match optional {
        // If `optional` destructures, evaluate the block.
        Some(i) => {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // ^ Requires 3 indentations!
        },
        // Quit when the destructure fails, meaning `break`.
        _ => { break; }
        // ^ Why should this be required? Seems superfluous.
    }
}

while let可以让这个match序列变得更好：
*/


fn main() {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    // `while let`解构`optional`，否则就`break`
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
        // 不需要显式的处理失败的情况
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
    // `if let`有额外的`else/else if`，而while let不需要这些
}
