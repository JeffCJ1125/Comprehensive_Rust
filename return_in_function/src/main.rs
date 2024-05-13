fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}
// This is the same as the above function, but it returns early if b > 0
fn gcd_return_early(a: u32, b: u32) -> u32 {
    if b > 0 {
        //use semi-colon after return statement to separate the return value from the block
        return gcd(b, a % b);
    }
    a
}

fn main() {
    println!("gcd: {}", gcd(143, 52));
    println!("gcd_return_early: {}", gcd_return_early(143, 52));
}
