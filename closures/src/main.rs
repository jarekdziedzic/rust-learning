use std::collections::HashMap;

struct Cacher<T, U, V>
    where
        T: Fn(U) -> V,
        T: Copy,
        U: Eq,
        U : Copy,
        U: std::hash::Hash
{
    calculation: T,
    values : HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
    where
        T: Fn(U) -> V,
        T: Copy,
        U: Eq,
        U: Copy,
        U: std::hash::Hash
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new()
        }
    }

    fn value(&mut self, arg: U) -> &V {
        // This kind of sucks. Why do I have to copy the closure?
        let calc = self.calculation.clone();
        self.values.entry(arg.clone()).or_insert_with(|| calc(arg))
    }
}

fn lazy_execution() {
    let mut cacher = Cacher::new(|x| x+1);
    println!("value(1) = {}", cacher.value(1));
    println!("value(2) = {}", cacher.value(2));
}

fn closures_101() {
    let hello_closure = |name| {
        println!("Hello {}!", name);
    };

    let hello_closure_annotated = |name : &str| -> () {
        println!("Hello {}!", name);
    };

    let hello_closure_minimal = |name| println!("Hello {}!", name);



    hello_closure("Jarek");
    hello_closure_annotated("Monika");
    // first call to a closure fixes the types
    hello_closure_minimal("Emilia");
    // this won't compile because the closure type is fixed at &str. It would work if it wasn't for
    // the line above
    // hello_closure_minimal(5);
}

fn closures_capture() {
    let x = vec![1,2];
    // let closure: FnMut(_) -> bool = move |z| z == x;
    closure(vec![1,2]);
    println!("x = {:?}", x);

}

fn main() {
    closures_101();
    lazy_execution();
    closures_capture();
}
