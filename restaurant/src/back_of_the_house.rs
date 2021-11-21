// you don't declare module here. It's done by file/dir hierarchy.
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

// The enum variants follow the visibility setting of the enum they're part of
pub enum Appetiser {
    Soup,
    Salad
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

mod waste {
    enum Kind {
        Plastics,
        Glass,
        Paper,
        General,
    }
}