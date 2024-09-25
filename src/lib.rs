use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use std::fs::File;
use std::io::*;

// pub fn run() -> Result<(), Box<dyn Error>> {
pub fn run() {
    let config = parse_args();

    let query = config.get_one::<String>("query").unwrap();
    let re = Regex::new(query).unwrap();
    let input = config.get_one::<String>("file").unwrap();

    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    let case_insensitive = config.get_flag("ignore_case");

    process_lines(reader, re, case_insensitive);
}

pub fn parse_args() -> ArgMatches {
    Command::new("minigrep")
        .version("0.1")
        .about("Search for a pattern in a file")
        .arg(
            Arg::new("query")
                .help("The pattern to search for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("file")
                .help("The file to search")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("ignore_case")
                .short('i')
                .long("ignore-case")
                .help("Case insensitive search")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex, ci: bool) {
    for line_ in reader.lines() {
        // Handle potential IO errors when reading the line
        let mut line = match line_ {
            Ok(line) => line,   // Successfully read the line
            Err(_) => continue, // Skip this iteration if an error occurs
        };

        // Convert to lowercase if `ci` (case-insensitive flag) is true
        if ci {
            line = line.to_lowercase();
        }

        // Check if the line matches the regex pattern
        if re.find(&line).is_some() {
            // Print the matching line
            println!("{}", line);
        }
    }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";
//
//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }
//
//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";
//
//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     }
// }
