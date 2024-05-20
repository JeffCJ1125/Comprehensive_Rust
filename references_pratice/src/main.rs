fn main() {
    println!("Hello, world!");

    let a = 'A';
    let b = 'B';
    let mut r = &a;
    println!("r : {r} ");
    println!("r : {}", r);
    println!("r : {}", *r);
    println!("r : {r} , a : {a} , b : {b}");
    r = &b;
    println!("r : {r} , a : {a} , b : {b}");

    let mut a = 'A';
    let mut b = 'B';
    a = 'a';
    let x = &mut a;
    let mut y = &mut b;
    println!("x : {x} , y : {y}");
    *x = *y;
    println!("x : {x} , y : {y}");
    let mut c = 'C';
    y = &mut c;
    println!("x : {x} , y : {y}");
}
