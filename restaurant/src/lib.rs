mod back_of_the_house;
mod front_of_house;

// external users of this module can now call add_to_waitlist directly. Due to the `pub`.
pub use front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // note back_of_the_house module is not public!
    let mut meal = back_of_the_house::Breakfast::summer("rye");
    meal.toast = String::from("Wheat");
    // Won't compile. It's private member of the struct!
    // meal.seasonal_fruit = String::from("Bluberries");

    let _appetiser = crate::back_of_the_house::Appetiser::Salad;

    // not needed here
    // use front_of_house;
    use front_of_house::cashier as clerk;
    clerk::handle_payment();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
