fn largest<T:std::cmp::PartialOrd + Copy>(list : &[T]) -> T {
    let mut largest = list[0];

    for &i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn smallest<T>(list : &[T]) -> T
    where T : std::cmp::PartialOrd + Copy
{
    let mut smallest = list[0];

    for &i in list {
        if i < smallest {
            smallest = i;
        }
    }
    smallest
}

#[derive(Debug)]
struct Point<T> {
    x : T,
    y : T,
}

trait OriginDistance<T> {
    fn distance_from_origin(&self) -> T;
}

impl OriginDistance<f32> for Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// accept trait (interface) as parameter. Trait is templatised but doesn't have to be
fn calculate_distance_from_origin<T>(item: &impl OriginDistance<T>) -> T {
    item.distance_from_origin()
}

trait OneI32Coordinate {
    fn first_coordinate(&self) -> i32;
}

impl OneI32Coordinate for Point<i32> {
    fn first_coordinate(&self) -> i32 {
        self.x
    }
}

trait SquareOfFirstCoordinate {
    fn first_coordinate_square(&self) -> i32;
}

impl<T: OneI32Coordinate> SquareOfFirstCoordinate for T {
    fn first_coordinate_square(&self) -> i32 {
        self.first_coordinate().pow(2)
    }
}

fn get_first_i32_coordinate(item : &impl OneI32Coordinate) -> i32 {
    item.first_coordinate()
}
