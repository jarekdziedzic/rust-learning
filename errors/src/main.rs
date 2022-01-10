#![allow(unused)]

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn username_or_panic() -> String {
    read_username_from_file().expect("Error getting username")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = read_username_from_file();
    println!("Result = {:?}", res);
    username_or_panic();
    File::open("notthere.txt")?;
    Ok(())
}