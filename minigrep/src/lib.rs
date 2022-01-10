extern crate core;
use std::fs;

pub struct MiniGrepArgs {
    query: String,
    filename: String,
}

impl MiniGrepArgs {
    pub fn new(query : String, filename : String) -> MiniGrepArgs {
        MiniGrepArgs { query, filename }
    }
}

pub fn parse(mut args: std::env::Args) -> Result<MiniGrepArgs, String> {
    let query;
    let filename;

    args.next(); //ignore executable name
    match args.next() {
        Some(v) => query = v,
        None => {return Err(String::from("Not enough arguments"));},
    }

    match args.next() {
        Some(v) => filename = v,
        None => {return Err(String::from("Not enough arguments"));},
    }

    return Ok(MiniGrepArgs::new(query, filename));
}

pub fn search(args: MiniGrepArgs) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = fs::read_to_string(args.filename)?;
    let res = search_in_string(&file, &args.query)?;
    // FIXME: Why can't I use Vec::map?
    let mut rv:Vec<String> = Vec::new();
    for r in res {
        rv.insert(rv.len() , String::from(r));
    }
    Ok(rv)
}

pub fn search_in_string<'a>(contents: &'a str, query: &'a str) -> Result<Vec<&'a str>, Box<dyn std::error::Error>> {
    let re = regex::Regex::new(query)?;
    let filtered = contents
        .lines()
        .filter(|line| re.is_match(line))
        .collect();
    Ok(filtered)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result() {
        let query = "a";
        let contents = "\
        aa\n\
        bb\n\
        cc\n";
        assert_eq!(vec!("aa"), search_in_string(contents, query).expect("Failed the search"));
    }

    #[test]
    fn no_results() {
        let query = "d";
        let contents = "\
        aa\n\
        bb\n\
        cc\n";

        assert_eq!(Vec::<&str>::new(), search_in_string(contents,query).expect("Failed to perform search"));
    }

    #[test]
    fn search_in_empty_string() {
        let query = "stuff";
        let contents = "";

        assert_eq!(Vec::<String>::new(), search_in_string(contents, query).expect("Failed to perform search"));
    }

    // This is shite, mocking env::Args is not trivial.
    // #[test]
    // fn query_and_filename_argument_are_accepted() {
    //     let correct_args = vec![String::from("exec"), String::from("file"), String::from("query")];
    //     assert!(parse(correct_args.iter()).is_ok());
    // }
    //
    // #[test]
    // fn not_enough_arguments_is_detected() {
    //     let args = vec![String::from("exec"), String::from("file")];
    //     assert!(parse(args.as_slice()).is_err());
    // }
}