use std::time::Duration;

fn main() {
    threads1();
    threads2();
    threads3();
}

fn threads1() {
    let handle = std::thread::spawn(|| {
        for i in 1..10 {
            println!("Thread 1 @{}", i);
            std::thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Thread 0 @{}", i);
        std::thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn threads2() {
    let v = vec![1,2,3];

    std::thread::spawn(move ||{
        println!("v = {:?}", v);
    });
}

fn threads3() {
    let (tx,rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let val = String::from("Hello!");
        tx.send(val).expect("Failed to send to a channel");
        println!("val is {}", val);
    });

    let result = rx.recv();
    println!("Result: {:?}", result);
}