use std::fs;
use regex;

pub struct MiniGrepArgs {
    query : String,
    filename : String,
}

pub fn parse( args : Vec<String>) -> Result<MiniGrepArgs, String> {
    if args.len() < 3 {
        return Err(String::from("Not enough arguments"));
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    return Ok(MiniGrepArgs{query, filename});
}

pub fn search(args : MiniGrepArgs) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = fs::read_to_string(args.filename)?;
    let re = regex::Regex::new(args.query.as_str())?;
    let filtered:Vec<String> = file.split("\n")
        .filter(|line| re.is_match(line))
        .map(|s| String::from(s))
        .collect();
    Ok(filtered)
}