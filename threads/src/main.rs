use std::time::Duration;

fn main() {
    threads1();
    threads2();
    threads3();
    threads4();
    threads5();
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
    });

    let result = rx.recv();
    println!("Result: {:?}", result);
}

fn threads4(){
    let (tx, rx) = std::sync::mpsc::channel();

    let tx1 = tx.clone();
    std::thread::spawn(move || {
        for elem in vec!["Item 1", "Item 2", "Item 3"] {
            tx1.send(elem).expect("Failed to send from tx1");
        }
    });

    let tx2 = tx.clone();
    std::thread::spawn(move || {
        for elem in vec!["Item 4", "Item 5", "Item 6"] {
            tx2.send(elem).expect("Failed to send from tx2");
        }
    });

    for _ in 1..6 {
        println!("Received {}", rx.recv().unwrap());
    }
}

fn threads5() {
    use std::sync::{Arc,Mutex};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let cloned_counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = cloned_counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}