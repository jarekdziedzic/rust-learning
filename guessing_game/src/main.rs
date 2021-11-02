mod scratch;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    const MAX_TRIES : u8 = 5;
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is {}.", secret_number);

    let mut tries = 0;
    loop {
        if tries == MAX_TRIES {
            println!("Game over!");
            break;
        }
        tries = tries + 1;

        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // ooops variable shadowing. Not sure I like it very much.
        // oops2: the type of guess influences the type of
        // secret_number because the two are compared
        // This sounds a little magical
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Whoah! I don't know what to make of that: {}", e);
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
        }
    }
}
