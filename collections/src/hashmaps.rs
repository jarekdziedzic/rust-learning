use std::collections::HashMap;

pub fn insert_example() {
    let mut colours = HashMap::new();
    colours.insert("red", 0xFF0000);
    colours.insert("blue", 0x0000FF);
    colours.insert("yellow", 0xFFFF00);

    find_and_print_colour(&colours, "red");
    find_and_print_colour(&colours, "cyan");

    colours.entry("black").or_insert(0x0);
    colours.insert("black", 0xFF); // OOPS that will overwrite the existing key
    find_and_print_colour(&colours, "black");
    colours.entry("black").or_insert(0x0); // that won't fix it
    find_and_print_colour(&colours, "black");
    // This time round save the ref
    let black_entry = colours.entry("black").or_insert(0x0); // that won't fix it
    *black_entry = 0x0; // that fixes it!
    find_and_print_colour(&colours, "black");
}

fn find_and_print_colour(colours : &HashMap<&str, i32>, key : &str) {
    if let Some(val) = colours.get(key) {
        println!("Found {}! The value is 0x{:X}", key, val);
    } else {
        println!("Did not find {}", key);
    }
}
