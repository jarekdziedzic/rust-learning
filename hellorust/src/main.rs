fn main() {
    let rv = print_hello_and_return();
    println!("Returned {}", rv);
}

fn print_hello() {
    println!("Hello, world!");
}

fn print_hello_and_return() -> i32 {
    print_hello();
    return  0;
}