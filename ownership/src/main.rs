fn main() {
    let mut s1 = String::from("Hello");
    println!("{}", s1);
    // This does implicit move!!!
    // let s2 = s1;
    // s1 is "destroyed"

    change(&mut s1);
    // s1 is moved to takes_ownership, which returns it, at which point it's moved again to s2
    let s2 = takes_ownership(s1);
    // println!("{}", s1);
    let (s3, len) = calculate_length(s2);
    let len2 = calculate_length_by_ref(&s3);
    println!("{} {} {}", s3, len, len2);
    slices(&s3);
    println!("lowercase_count({}) = {}", s3, lowercase_count(&s3));
    println!("uppercase_count({}) = {}", s3, uppercase_count(&s3));
    println!("whitespace_count({}) = {}", s3, whitespace_count(&s3));
    println!("whitespace_count({}) = {}", "literal string", whitespace_count("literal string"));


}

fn takes_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_by_ref(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" beautiful world!");
}

fn slices(s: &String) {
    println!("First word in {} is {}", s, first_word(s));
    println!("2nd word in {} is {}", s, n_th_word(s,2));
    println!("3rd word in {} is {}", s, n_th_word(s,3));
    println!("4th word in {} is {}", s, n_th_word(s,4));

    let ss = "Ala ma kota";
    println!("First word in {} is {}", ss, first_word(ss));
    println!("2nd word in {} is {}", ss, n_th_word(ss,2));
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn n_th_word(s: &str, n: u32) -> &str {
    let bytes = s.as_bytes();
    let mut word: u32 = 0;
    let mut start_index: Option<usize> = None;
    let mut end_index: Option<usize> = None;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            word += 1;

            if word == (n - 1) {
                start_index = Some(i + 1);
            }

            if word == n {
                end_index = Some(i);
            }
        }
    }

    return match start_index {
        None => &s[0..0],
        Some(si) => {
            let ei = match end_index {
                None => s.len(),
                Some(res) => res,
            };
            &s[si..ei]
        },
    };
}

fn lowercase_count(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut count: u32 = 0;
    for (_, &item) in bytes.iter().enumerate() {
        if item >= b'a' && item <= b'z' {
            count += 1;
        }
    }
    count
}

fn uppercase_count(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut count : u32 = 0;
    for (_, &item) in bytes.iter().enumerate() {
        if item >= b'A' && item <= b'Z' {
            count += 1;
        }
    }
    count
}

fn whitespace_count(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut count : u32 = 0;
    for &x in bytes {
        if x == b' ' {
            count += 1;
        }
    }
    count
}