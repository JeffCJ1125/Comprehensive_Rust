// The Collatz Sequence is defined as follows, for an arbitrary n1 greater than zero:
// If ni is 1, then the sequence terminates at ni.
// If ni is even, then ni+1 = ni / 2.
// If ni is odd, then ni+1 = 3 * ni + 1.
// For example, beginning with n1 = 3:
// 3 is odd, so n2 = 3 * 3 + 1 = 10;
// 10 is even, so n3 = 10 / 2 = 5;
// 5 is odd, so n4 = 3 * 5 + 1 = 16;
// 16 is even, so n5 = 16 / 2 = 8;
// 8 is even, so n6 = 8 / 2 = 4;
// 4 is even, so n7 = 4 / 2 = 2;
// 2 is even, so n8 = 1; and the sequence terminates.
/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(n: i32) -> u32 {
    // todo!("Implement this")
    // println!("n : {}", n);
    if n == 1 {
        return 1;
    }
    let mut len = 1;
    if n % 2 == 0 {
        len += collatz_length(n / 2);
        return len;
    }
    len += collatz_length(3 * n + 1);
    len
}

fn collatz_length_solution(mut n: i32) -> u32 {
    // todo!("Implement this")
    let mut len = 1;
    while n > 1 {
        // println!("n : {}", n);
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    // todo!("Implement this")
    println!("collatz_length(3): {}", collatz_length(3));
    println!("collatz_length_solution(3): {}", collatz_length_solution(3));
}
