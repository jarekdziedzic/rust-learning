pub fn statistics() {
    let v  = vec![0,1,2,3,4,5,6,7,8,9];

    // calculate mean
    let mut sum = 0;
    for e in &v {
        sum += e;
    }
    let avg = sum as f64 / v.len() as f64;
    println!("Avg({:?}) = {}", v, avg)

    // TODO: calculate median

    // TODO: calculate mode

}