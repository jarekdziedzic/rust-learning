fn main() {
    const SEQ_NUM:u32 = 10;
    println!("fib({}) = {}", SEQ_NUM, fib(SEQ_NUM));
}

fn fib(n : u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
