use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::Arc;
use std::thread;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct RunParameters {
    pub case_insensitive: bool,
    pub line_numbers: bool,
    pub highlight: bool,
    pub all_text: bool,
}

pub fn run() -> Result<(), io::Error> {
    let config = parse_args();

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

    let query = config.get_one::<String>("query").unwrap();
    let final_query = if run_parameters.case_insensitive {
        query.to_lowercase() // Convert to lowercase if case-insensitive
    } else {
        query.clone() // Otherwise, keep the original
    };
    let re = Regex::new(final_query.as_str()).unwrap();

    if let Some(files) = config.get_many::<String>("file") {
        let run_parameters = Arc::new(run_parameters);
        let re = Arc::new(re);

        let mut handles = vec![];

        for file in files {
            let run_params = Arc::clone(&run_parameters);
            let regex = Arc::clone(&re);
            let file = file.to_string();

            // Spawn a new thread for each file
            let handle = thread::spawn(move || {
                let f = File::open(&file);
                match f {
                    Ok(f) => {
                        let reader = BufReader::new(f);
                        let output = process_lines(reader, &regex, &run_params);
                        println!("\nResults for file: {} \n{}", file, output.join("\n"));
                    }
                    Err(e) => {
                        eprintln!("Error opening file {}: {}", file, e);
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    } else {
        println!("no file found")
    }
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

pub fn process_lines<T: BufRead + Sized>(
    reader: T,
    re: &Regex,
    run_parameters: &RunParameters,
) -> Vec<String> {
    let mut line_number = 0;
    let mut output: Vec<String> = Vec::new();
    for line_ in reader.lines() {
        line_number += 1;
        let mat_line;
        // Handle potential IO errors when reading the line
        let line = match line_ {
            Ok(line) => {
                if run_parameters.case_insensitive {
                    mat_line = line.to_lowercase();
                } else {
                    mat_line = line.clone();
                }
                line
            } // Successfully read the line
            Err(_) => {
                continue;
            } // Skip this iteration if an error occurs
        };

        let mut highlighted_text = String::new();
        let mut include = false;
        let mut last_end = 0;

        let highlight_start = "\x1b[100m"; // Bright white background
        let highlight_end = "\x1b[0m"; // Reset formatting

        // If the line contains the search query, print it.
        for mat in re.find_iter(&mat_line) {
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
            (true, false) => output.push(highlighted_text.to_string()),
            (true, true) => output.push(format!("Line {}: {}", line_number, highlighted_text)),
            (false, true) => output.push(format!("Line {}: {}", line_number, line)),
            (false, false) => output.push(line.to_string()),
        }
    }
    output
}
