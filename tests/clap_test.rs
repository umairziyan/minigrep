use minigrep::*;
use regex::Regex;
use std::io::Cursor;

#[test]
fn test_case_sensitive_search() {
    let input = "Hello\nworld\nRust programming";
    let cursor = Cursor::new(input);
    let regex = Regex::new("Rust").unwrap();

    let params = RunParameters {
        query: regex,
        case_insensitive: false,
        line_numbers: false,
        highlight: false,
        all_text: false,
    };

    let result = process_lines(cursor, &params);
    assert_eq!(result, vec!["Rust programming"]);
}

#[test]
fn test_case_insensitive_search() {
    let input = "Hello\nworld\nrust programming";
    let cursor = Cursor::new(input);
    // This includes the case-insensitive flag for the input as the input will be converted to
    // lowercase before comparison.
    let regex = Regex::new("rust").unwrap();

    let params = RunParameters {
        query: regex,
        case_insensitive: true,
        line_numbers: false,
        highlight: false,
        all_text: false,
    };

    let result = process_lines(cursor, &params);
    assert_eq!(result, vec!["rust programming"]);
}

#[test]
fn test_highlighted_search() {
    let input = "Hello\nworld\nRust programming";
    let cursor = Cursor::new(input);
    let regex = Regex::new("Rust").unwrap();

    let params = RunParameters {
        query: regex,
        case_insensitive: false,
        line_numbers: false,
        highlight: true,
        all_text: false,
    };

    let highlight_start = "\x1b[100m";
    let highlight_end = "\x1b[0m";

    let result = process_lines(cursor, &params);
    assert_eq!(
        result,
        vec![format!(
            "{}Rust{} programming",
            highlight_start, highlight_end
        )]
    );
}

#[test]
fn test_search_with_line_numbers() {
    let input = "Hello\nworld\nRust programming";
    let cursor = Cursor::new(input);
    let regex = Regex::new("Rust").unwrap();

    let params = RunParameters {
        query: regex,

        case_insensitive: false,
        line_numbers: true,
        highlight: false,
        all_text: false,
    };

    let result = process_lines(cursor, &params);
    assert_eq!(result, vec!["Line 3: Rust programming"]);
}

#[test]
fn test_all_text_with_highlight() {
    let input = "Hello\nworld\nRust programming";
    let cursor = Cursor::new(input);
    let regex = Regex::new("Rust").unwrap();

    let params = RunParameters {
        query: regex,

        case_insensitive: false,
        line_numbers: false,
        highlight: true,
        all_text: true,
    };

    let highlight_start = "\x1b[100m";
    let highlight_end = "\x1b[0m";

    let result = process_lines(cursor, &params);
    println!("{:?}", result);
    assert_eq!(
        result,
        vec![
            "Hello".to_string(),
            "world".to_string(),
            format!("{}Rust{} programming", highlight_start, highlight_end)
        ]
    );
}

#[test]
fn test_no_matches() {
    let input = "Hello\nworld\nRust programming";
    let cursor = Cursor::new(input);
    let regex = Regex::new("Java").unwrap();

    let params = RunParameters {
        query: regex,

        case_insensitive: false,
        line_numbers: false,
        highlight: false,
        all_text: false,
    };

    let result = process_lines(cursor, &params);
    assert!(result.is_empty());
}
