/*
对于一些情况，match是很不方便的。比如：

// Make `optional` of type `Option<i32>`
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ Needed 2 indentations just so we could destructure
        // `i` from the option.
    },
    _ => {},
    // ^ Required because `match` is exhaustive. Doesn't it seem
    // like wasted space?
};



*/

fn main() {
    // All have type `Option<i32>`
    // Option类型
    let number   = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`). Else do nothing.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    };

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluated the condition to see if this branch
    // should be taken.
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    // The condition evaluated false. This branch is the default.
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    };
}
