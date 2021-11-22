
pub fn vector_create() {
    println!("=== vector_create ===");
    let v = vec!["ala", "ma", "kota"];

    for (i, e) in v.iter().enumerate() {
        println!("v[{}] = {}", i, e);
    }

    // what's the difference here between 'in v' vs 'in &v'? Is v copied?
    for e in &v {
        println!("Found element {} ", e);
    }

    let mut v2 : Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    println!("v2 = {:#?}", v2);
    // This will panic. This is essentially a logical error. Won't go unchecked.
    //println!("v2[100] = {}", v2[100]);
}

pub fn vector_refs() {
    println!("=== vector_refs ===");
    let mut vector = vec![1,2,3,4,5];
    let v = &vector[2];
    println!("v = {}", v);

    // this is fine with the borrow checker, because the non mutable reference is not used beyond
    // this point
    vector.push(6);

    // This would fail to compile because the non mutable reference is used beyond the mutable ref
    // println!("v = {}", v);
}
