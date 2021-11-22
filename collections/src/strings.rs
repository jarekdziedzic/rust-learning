pub fn str_concat() {
    let mut s = String::from("hello");
    s.push(' ');
    let s2 = String::from("world");

    let s3 = s + &s2;
    println!("s3 = {}", &s3);
    // operator + moves the self (left hand) argument. This won't compile
    //println!("s = {}", &s);

    for c in s3.chars() {
        print!("{},", c);
    }

    println!();
    let s4 = format!("{}{}", &s3, "!");
    println!("{}", s4);
    let s5 = format!("{}{}", &s3, '!');
    println!("{}", s5);

    let mut s6 = s3;
    s6.push_str("!");
    println!("s6 = {}", s6);
    // println!("s3 = {}", s3);
}