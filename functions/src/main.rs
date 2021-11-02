fn main() {
    greet(String::from("Jarek"), 38);
    let total = price(6);
    price(8);
}

fn greet(name : String, age: u8) {
    println!("Hello {}! Is your age {}?", name, age);
}

fn price(width: i32) -> i32 {
    let price = if width == 6 { 65 } else { 85 };
    println!("The price for width {} is {}", width, price);
    price
}