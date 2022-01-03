use minigrep;
use std::env;

fn main() {
    let argv = env::args().collect();
    let args = minigrep::parse(argv)
        .unwrap_or_else(|err| {
            println!("Failed to parse args: {}", err);
            std::process::exit(1);
        }
    );
    let matches = minigrep::search(args)
        .unwrap_or_else(|err| {
            println!("Failed to perform search: {}", err);
            std::process::exit(2);
        });
    println!("Matches: {:#?}", matches);
}
