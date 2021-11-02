fn main() {
    let mut counter = 0;
    let rv = loop {

        println!("again!");
        counter = counter + 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("rv = {}", rv);
    liftoff(4);
    liftoff2(3);

    collection_loop();
}


fn liftoff(mut timeout : u32) {
    while timeout > 0 {
        println!("{}", timeout);
        timeout = timeout - 1;
    }
    println!("LIFTOFF!!!");
}

fn liftoff2(timeout : u32)
{
    for t in (1..timeout + 1).rev() {
        println!("{}", t);
    }
    println!("LIFTOFF");
}

fn collection_loop() {
    let numbers = [1, 2, 3, 4, 5];

    for n in numbers {
        println!("number {}", n);
    }
}