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
        // take the rest of the fields from u1. It would be destructive
        // for u1 if any of the remaining fields didn't implement the
        // copy trait. Copy trait basically means that the object
        // is copied rather than moved on assignment
        ..u1
    };

    // let u4 = User {
    //     email: String::from("example@gmail.com"),
    //     ..u1 // this moves the fields from u1
    // };

    println!("Name {}, email: {}, age: {}", u1.name, u1.email, u1.age);
    println!("Name {}, email: {}, age: {}", u2.name, u2.email, u2.age);

    shapes_main();
    tuple_structs_main();
    dbg_main();
}

trait Area {
    fn area(&self) -> u32;
}

#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

impl Area for Rectangle {
    fn area(self: &Rectangle) -> u32 {
        self.width * self.height
    }
}

fn shapes_main() {
    let r1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Area of {:#?} is {}", &r1, r1.area());

    let c1 = Circle {
        radius: 10
    };
    println!("Area of {:?} is {}", &c1, c1.area());

    let c2 = make_circle(5);
    println!("Area of {:?} is {}", c2, c2.area());
}

#[derive(Debug)]
struct Circle {
    radius : u32,
}

impl Area for Circle {
    fn area(self : &Circle) -> u32 {
        //FIXME: Can this be made less ugly?
        (3.1415 * (self.radius as f64) * (self.radius as f64)) as u32
    }
}


// pointless fn really, except for that it shows the syntax for returning struct objects
fn make_circle(radius : u32) -> Circle {
    Circle { radius }
}

#[derive(Debug)]
struct RGB(u8, u8, u8);

fn tuple_structs_main() {
    let black = RGB(0,0,0);
    let white = RGB(255,255,255);
    let red = RGB(255,0,0);
    println!("Black is {:?}", black);
    println!("White is {:?}", white);
    println!("Red is {:?}", red);
}

#[derive(Debug)]
struct RGBA(u8, u8, u8, u8);

fn dbg_main() {
    let full_opacity  = 255;
    // this does not require Debug attribute
    let black_half_opaque = RGBA(0,0,0,dbg!(full_opacity/2));
    // but this one does!
    // note that dbg macro takes ownership of the argument, and then returns it back.
    dbg!(black_half_opaque);
    // oops that won't work!
    // println!("{:?}", black_half_opaque);
    let black_opaque = RGBA(0,0,0,full_opacity);
    // pass by reference. That will work.
    // note that dbg! prints to stderr, and println! prints to stdout
    dbg!(&black_opaque);
    // TODO: Find out why println! doesn't move the args, and dbg! does? What's the logic?
    println!("{:?}", black_opaque);
    println!("{:?}", &black_opaque);
}