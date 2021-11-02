#[derive(Clone,Debug)]
struct User {
    name : String,
    email : String,
    age : u8,
}

fn main() {
    let u1 = User {
        email:String::from("bob.satan@gmail.com"),
        name:String::from("Bob Satan"),
        age:66,
    };


    let u2 = User {
        email: String::from("ave.cesar@gmail.com"),
        ..u1.clone() // moves the fields from a temp copy of u1. Leaves u1 intact
    };

    let _u3 = User {
        email: u1.email.clone(),
        name: u1.name.clone(),
        ..u1
    };

    // let u4 = User {
    //     email: String::from("example@gmail.com"),
    //     ..u1 // this moves the fields from u1
    // };

    println!("Name {}, email: {}, age: {}", u1.name, u1.email, u1.age);
    println!("Name {}, email: {}, age: {}", u2.name, u2.email, u2.age);

    rectangle_main();
}

#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

fn area( r : &Rectangle) -> u32 {
    r.width * r.height
}

fn rectangle_main() {
    let r1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Area of {:#?} is {}", &r1, area(&r1));
}