use std::ops::Deref;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn cons() {
    use List::*;
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:?}", list);
}


#[derive(Debug)]
enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

fn cons2() {
    use List2::*;
    let l1 = Rc::new(Cons(1, Rc::new(Cons(2,Rc::new(Nil)))));
    let l2 = Rc::new(Cons(0, Rc::clone(&l1)));
    let l3 = Rc::new(Cons(1, Rc::clone(&l1)));
    println!("l2 = {:?}", l2);
    println!("l2 strong_count: {}", Rc::strong_count(&l2));
    println!("l1 strong_count: {}", Rc::strong_count(&l1));
    println!("l1 weak_count: {}", Rc::weak_count(&l1));
    println!("l3 = {:?}", l3);
}

#[derive(Debug)]
struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(val : T) -> MyBox<T> {
        MyBox(val)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T>
{
    fn drop(&mut self) {
        println!("Dropping MyBox");
    }
}

fn my_box() {
    let bi = MyBox::new(5);
    println!("{:?} == 5? {}", bi, *bi == 5);
    // call destructor early. Won't be called again at the end of scope.
    drop(bi);
    let bs = MyBox::new(String::from("deref"));
    // deref coercion in action
    let s: &str = &bs;
    println!("s == {}", s);
}

fn main() {
    cons();
    cons2();
    my_box();
}
