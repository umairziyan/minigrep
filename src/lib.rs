use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::thread;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct RunParameters {
    pub case_insensitive: bool,
    pub line_numbers: bool,
    pub highlight: bool,
    pub all_text: bool,
}

impl RunParameters {
    pub fn from_config(config: &ArgMatches) -> Self {
        let all_text = config.get_flag("all_text");
        Self {
            case_insensitive: config.get_flag("ignore_case"),
            line_numbers: config.get_flag("line_numbers"),
            highlight: all_text || config.get_flag("highlight"),
            all_text,
        }
    }
}

pub fn run() -> Result<(), io::Error> {
    let config = parse_args();
    let run_parameters = RunParameters::from_config(&config);

    let query = config.get_one::<String>("query").unwrap();
    let final_query = if run_parameters.case_insensitive {
        query.to_lowercase() // Convert to lowercase if case-insensitive
    } else {
        query.clone() // Otherwise, keep the original
    };
    let re = Regex::new(final_query.as_str()).unwrap();

    if let Some(files) = config.get_many::<String>("file") {
        let handles: Vec<_> = files
            .map(|file| {
                let run_params = run_parameters.clone();
                let regex = re.clone();
                let file = file.to_string();

                thread::spawn(move || process_file(file, regex, run_params))
            })
            .collect();

        for handle in handles {
            if let Err(e) = handle.join().expect("Thread panicked") {
                println!("Error processing file: {}", e);
            }
        }
    } else {
        println!("No file found");
    }
    Ok(())
}

/// Process a file and display the results
fn process_file(file: String, re: Regex, run_parameters: RunParameters) -> Result<(), io::Error> {
    let f = File::open(&file)?;
    let reader = BufReader::new(f);
    let results = process_lines(reader, &re, &run_parameters);
    println!("\nResults for file: {} \n{}", file, results.join("\n"));
    Ok(())
}

/// Process a line in the file and return the results
pub fn process_lines<T: BufRead + Sized>(
    reader: T,
    re: &Regex,
    run_parameters: &RunParameters,
) -> Vec<String> {
    reader
        .lines()
        .enumerate()
        .filter_map(|(line_number, line)| line.ok().map(|content| (line_number + 1, content)))
        .filter_map(|(line_number, line)| {
            let mat_line = if run_parameters.case_insensitive {
                line.to_lowercase()
            } else {
                line.clone()
            };
            let highlighted_line = match highlight_matches(&line, &mat_line, re, run_parameters) {
                Some(text) => text,
                // Skip this line if all_text isn't selected.
                None => {
                    if !run_parameters.all_text {
                        return None;
                    } else {
                        line.to_string()
                    }
                }
            };
            // Add line numbers if requested
            let line_prefix = if run_parameters.line_numbers {
                format!("Line {}: ", line_number)
            } else {
                String::new()
            };
            Some(format!("{}{}", line_prefix, highlighted_line))
        })
        .collect()
}
/// Highlights regex matches in the given line if requested.
/// If highlighting is disabled, returns the original line.
/// Returns None if no matches are found.
fn highlight_matches(
    line: &str,
    updated_line: &str,
    re: &Regex,
    params: &RunParameters,
) -> Option<String> {
    // Early return if no match is found
    if !re.is_match(updated_line) {
        return None;
    }

    // If highlighting is disabled, return the original line
    if !params.highlight {
        return Some(line.to_string());
    }

    const HIGHLIGHT_START: &str = "\x1b[100m";
    const HIGHLIGHT_END: &str = "\x1b[0m";

    // Perform replacements and highlight matches
    Some(
        re.replace_all(updated_line, |caps: &regex::Captures| {
            caps.get(0).map_or_else(String::new, |m| {
                format!(
                    "{}{}{}",
                    HIGHLIGHT_START,
                    &line[m.start()..m.end()],
                    HIGHLIGHT_END
                )
            })
        })
        .into_owned(),
    )
}

/// Parses the command-line arguments for the `minigrep` program.
pub fn parse_args() -> ArgMatches {
    Command::new("minigrep")
        .version("0.1")
        .about("Search for a pattern in a file")
        .arg(
            Arg::new("query")
                .help("The pattern to search for")
                .required(true)
        )
        .arg(
            Arg::new("file")
                .help("The file/s to search, multiple files can be added by separating them with a space, e.g. file1.txt, file2.txt")
                .required(true)
                .num_args(1..)
                .value_delimiter(' ')
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
