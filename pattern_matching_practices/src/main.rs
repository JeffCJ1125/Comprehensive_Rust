#[rustfmt::skip]
fn main() {
    let input = 'x';
    // Match can be used as an expression. Just like if, each match arm must have the same type.
    // The type is the last expression of the block, if any. In the example above, the type is ().
    let res = match input {
        'q'                       => {println!("Quitting"); 3},
        'a' | 's' | 'w' | 'd'     => {println!("Moving around");4},
        '0'..='9'                 => {println!("Number input");2},
        key if key.is_lowercase() => {println!("Lowercase: {key}"); 1},
        _                         => {println!("Something else"); 0},
    };
    println!("res : {res}");

    let test = Foo { x: (2, 2), y: 4 };
    match test {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
}

struct Foo {
    x: (u32, u32),
    y: u32,
}
