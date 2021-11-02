use std;

fn main() {
    println!("Enter temperature, eg. 32C or 0F: ");
    let mut temp_in = String::new();
    std::io::stdin().read_line(&mut temp_in).expect("Failed to read input");
    let temp_fahrenheit :f32;
    let temp_celsius :f32;

    let f_loc = temp_in.find("F");
    let c_loc = temp_in.find("C");
    if f_loc.is_some() {
        temp_fahrenheit = match temp_in.trim().strip_suffix("F"){
            Some(stripped) => stripped.parse().expect("This is not an integer"),
            None => panic!("This shouldn't really happen"),
        };
        temp_celsius = (temp_fahrenheit - 32.0) * 5.0/9.0;
    }
    else if c_loc.is_some() {
        temp_celsius = match temp_in.trim().strip_suffix("C") {
            Some(stripped) => stripped.parse().expect("This is not an integer"),
            None => panic!("This shouldn't really happen"),
        };
        temp_fahrenheit =  temp_celsius * 1.8 + 32.0;
    }
    else {
        panic!("Didn't understand the temperature.");
    }

    println!("You entered {}. This will be {} degrees C and {} degrees F",
             temp_in.trim(), temp_celsius, temp_fahrenheit);
}
