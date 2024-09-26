use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Default)]
struct RunParameters {
    case_insensitive: bool,
    line_numbers: bool,
    highlight: bool,
    all_text: bool,
}

pub fn run() -> Result<(), io::Error> {
    let config = parse_args();

    let query = config.get_one::<String>("query").unwrap();
    let re = Regex::new(query).unwrap();
    let input = config.get_one::<String>("file").unwrap();
    let f = File::open(input)?;

    let reader = BufReader::new(f);

    let run_parameters = RunParameters {
        case_insensitive: config.get_flag("ignore_case"),
        line_numbers: config.get_flag("line_numbers"),
        // If all text is shown, automatically set highlight matching.
        highlight: if config.get_flag("all_text") {
            true
        } else {
            config.get_flag("highlight")
        },
        all_text: config.get_flag("all_text"),
    };

    process_lines(reader, re, run_parameters);
    Ok(())
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
        .arg(
            Arg::new("line_numbers")
                .short('l')
                .long("line-numbers")
                .help("Display line numbers")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("highlight")
                .short('m')
                .long("highlight")
                .help("Highlight matches")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("all_text")
                .short('a')
                .long("all-text")
                .help("Print all document, if this is enabled, matches will automatically be highlighted.")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex, run_parameters: RunParameters) {
    let mut line_number = 0;
    for line_ in reader.lines() {
        line_number += 1;
        // Handle potential IO errors when reading the line
        let mut line = match line_ {
            Ok(line) => line, // Successfully read the line
            Err(_) => {
                continue;
            } // Skip this iteration if an error occurs
        };

        // Convert to lowercase if `ci` (case-insensitive flag) is true
        if run_parameters.case_insensitive {
            line = line.to_lowercase();
        }

        let mut highlighted_text = String::new();
        let mut include = false;
        let mut last_end = 0;

        let highlight_start = "\x1b[100m"; // Bright white background
        let highlight_end = "\x1b[0m"; // Reset formatting

        // If the line contains the search query, print it.
        for mat in re.find_iter(&line) {
            // Add the text before the match
            highlighted_text.push_str(&line[last_end..mat.start()]);

            // Add the highlighted match
            highlighted_text.push_str(&format!(
                "{}{}{}",
                highlight_start,
                &line[mat.start()..mat.end()],
                highlight_end
            ));

            // Update last_end to the end of the current match
            last_end = mat.end();
            include = true;
        }

        // Add any remaining text after the last match
        highlighted_text.push_str(&line[last_end..]);

        if !run_parameters.all_text && !include {
            continue;
        }

        match (run_parameters.highlight, run_parameters.line_numbers) {
            (true, false) => println!("{}", highlighted_text),
            (true, true) => println!("Line {}: {}", line_number, highlighted_text),
            (false, true) => println!("Line {}: {}", line_number, line),
            (false, false) => println!("{}: {}", line_number, line),
        }
    }
}
