<p align="center">
  <img src="https://github.com/umairziyan/minigrep/actions/workflows/rust.yml/badge.svg" alt="Rust">
</p>
# minigrep

A simple version of `grep` (**g**lobally search a **r**egular **e**xpression and
**p**rint). In the simplest use case, `grep` searches a specified file for a specified string (it allows regex patterns). Minigrep accepts two arguments: a string and a file path. It reads the file and searches for lines that contain the specified string, then prints those matching lines.

## Table of Contents

- [Build](#build)
- [Usage](#usage)
- [Potential Features](#potential-features)
- [License](#license)

## Build

minigrep is written in Rust, so you'll need to install [Rust](https://www.rust-lang.org/) to compile it. It will compile with the latest version of Rust.

To build minigrep:

```sh
git clone https://github.com/umairziyan/minigrep
cd minigrep
cargo build --release
```

## Usage

Change directory to the location of the `mingrep` binary, normally stored in:

```sh
/target/release/minigrep
```

To run minigrep, execute the binary with the search string and the file to search:

```sh
./minigrep text file.txt
```

```sh

Search for a pattern in a file

Usage: minigrep [OPTIONS] <query> <file>...

Arguments:
  <query>    The pattern to search for
  <file>...  The file/s to search, multiple files can be added by separating them with a space, e.g. file1.txt, file2.txt

Options:
  -i, --ignore-case   Case insensitive search
  -l, --line-numbers  Display line numbers
  -m, --highlight     Highlight matches
  -a, --all-text      Print all document, if this is enabled, matches will automatically be highlighted.
  -h, --help          Print help
  -V, --version       Print version

Examples:

  minigrep "error" file1.txt                        The pattern "error" in `file1.txt`
  minigrep "warning" file1.txt file2.txt            Search for "warning" in `file1.txt` and `file2.txt`
  minigrep -i "network" file1.txt                   Search for "network" in `file1.txt` ignoring case
  minigrep -l "timeout" file1.txt                   Show line numbers for "timeout" matches in `file1.txt`
  minigrep -m "failed" file1.txt                    Highlight occurrences of "failed" in `file1.txt`
  minigrep -a "response" file1.txt file2.txt        Print full content with "response" highlighted
  minigrep -i -l "disk" file1.txt                   Case insensitive search with line numbers for "disk"
  minigrep -m -l "memory" file1.txt                 Highlight "memory" and show line numbers in `file1.txt`
  minigrep -i -m "status" file1.txt                 Case insensitive search for "status" with highlighting
  minigrep -a -l -m "server" file1.txt file2.txt    Print entire file, highlight "server", and show line numbers

```

## Potential Features

~~**Case Insensitivity:** Add an option to perform case-insensitive searches. You could use a command-line argument to toggle this feature.~~

~~**Regex Support:** Integrate regular expressions to allow for more complex search patterns.~~

~~**Line Number Display:** Modify the output to show line numbers along with the matching lines.~~

~~**Search in Multiple Files:** Expand the functionality to search across multiple files or directories, rather than just a single file.~~

~~**Highlight Matches:** Implement a way to highlight or emphasize matching text in the output to make it easier to spot.~~

~~**Performance Improvements:** Optimize the search algorithm or implement parallel processing to handle large files or directories more efficiently.~~

**Interactive Mode:** Create an interactive mode where users can input search queries and navigate through results more dynamically.

**Customizable Output Formatting:** Allow users to customize the format of the output, such as showing matches in a specific color or format.

**File Type Filtering:** Add a feature to filter files by type.

**Support for Binary Files:** Add an option to handle binary files or to skip them entirely during the search.
