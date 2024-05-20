fn main() {
    let arr1: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("arr1[3] = {}", arr1[3]);
    let mut arr1: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("arr1[3] = {}", arr1[3]);
    arr1[3] = 5;
    println!("arr1[3] = {}", arr1[3]);
    println!("arr1 = {:?}", arr1);

    let slice1 = &mut arr1[2..4];
    println!("slice1 = {:?}", slice1);
    slice1[1] = 6;
    println!("slice1 = {:?}", slice1);
    arr1[1] = 3;
    println!("arr1 = {:?}", arr1);

    let mut arr2: [i32; 3] = [1, 2, 3];

    let mut slice2 = &mut arr2[1..];
    slice2[0] = 0;
    println!("slice2 = {:?}", slice2);
    let mut arr3: [i32; 4] = [3, 2, 1, 0];
    slice2 = &mut arr3[..];
    println!("slice2 = {:?}", slice2);
    let slice2 = &arr3;
    println!("slice2 = {:?}", slice2);

    let str_literal = "string slice (&str) can not be changed";
    println!("{str_literal}");
    let partial_str = &str_literal[6..];
    println!("we can keep slice the string slice");
    println!("&str_liter[6..] = {partial_str}");

    let str1 = String::new();
    println!("str1 = String::new() -> {str1}");
    let str2 = String::from("value");
    let mut str3 = str2.clone();
    println!("str2 = {str2}");
    println!("str3 = {str3}");
    str3 = str1;
    println!("str3 = {str3}");

    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}
