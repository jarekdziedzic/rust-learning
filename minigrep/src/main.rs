use std::env;

fn main() {
    let args = minigrep::parse(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse args: {}. \nUsage {} QUERY FILENAME", err, env::args().next().expect("Missing executable name."));
        std::process::exit(1);
    });
    let matches = minigrep::search(args).unwrap_or_else(|err| {
        eprintln!("Failed to perform search: {}", err);
        std::process::exit(2);
    });
    println!("Matches: {:#?}", matches);
}
