fn longest<'b>(x : &'b str, y : &'b str) -> &'b str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn shortest<'a>(x : &'a str, y : &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'a str) -> &'a str {
        if announcement.len() > self.part.len() {
            println!("Attention please: {}", announcement);
            announcement
        } else {
            self.part
        }
    }
}

fn main() {
    let x = String::from("Hello");
    let y = String::from("There");
    let mut result;
    {
        result = longest(x.as_str(), y.as_str());
    }
    println!("The longest string is {}", result);

    result = shortest(x.as_str(), y.as_str());

    println!("The shortest string is {}", result);

    let excerpt = ImportantExcerpt{ part : x.as_str()};
    excerpt.announce_and_return_part(y.as_str());
}
