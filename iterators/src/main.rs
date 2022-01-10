fn sum() {
    let v = vec![1,2,3];
    let mut vi = v.iter();

    println!("{}", vi.next().expect("empty iterator"));

    let total: u32 =  vi.sum();
    println!("sum = {}", total);
}

fn map() {
    let v = vec![1,2,3];

    let w :Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("w = {:?}", w);
    println!("v = {:?}", v);
}

fn filter() {
    let v = vec![1,2,3,4];


    println!("v = {:?}, even v = {:?}",
             v,
             // filter takes a reference to the iterator rather than iterator itself
             // as the self argument. This is why &x is used in the closure argument.
            // An alternative would be to drop the ref and deref in the closure body,
             // i.e. (*x %2) == 0
             v.iter().filter(|&x| (x % 2) == 0).collect::<Vec<_>>()
    );
}

fn main() {
    sum();
    map();
    filter();
}
