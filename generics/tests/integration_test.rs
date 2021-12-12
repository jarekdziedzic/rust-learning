use linked_list;

#[test]
fn test_linked_list() {
    linked_list::test_lists();
}

#[test]
fn test_point() {
    let v = vec![3,4,5,1,2];

    println!("{:?}: largest {}", &v, largest(&v));
    println!("{:?}: smallest {}", &v, smallest(&v));

    let p0 = Point{ x: 3, y: 0};
    let fp1 = Point{ x: 1.0, y: 1.0};
    println!("p0 = {:?}, first_coordinate = {:?}, square = {}", p0,
             get_first_i32_coordinate(&p0),
             p0.first_coordinate_square() );
    println!("fp1 = {:?}, distance = {:?}", fp1, fp1.distance_from_origin() );
    println!("fp1 = {:?}, distance = {:?}", fp1, calculate_distance_from_origin(&fp1));
}