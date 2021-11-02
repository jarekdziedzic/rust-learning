fn main() {
    let x = {
        let y = 5;
        2*y
    };
    println!("The value of x: {}", x);

    let z = if x < 10 { 1000 } else { 5000 };

    println!("The value of z: {}", z);
    // that won't work because types have to be the same
    // let oops = if x < 10 { 1000 } else { "dupa" };
}
