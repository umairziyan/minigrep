# minigrep

A simple version of `grep` (**g**lobally search a **r**egular **e**xpression and
**p**rint). In the simplest use case, `grep` searches a specified file for a specified string. Minigrep accepts two arguments: a string and a file path. It reads the file and searches for lines that contain the specified string, then prints those matching lines.

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
Usage: minigrep <search> <file> [options]

Commands:
    minigrep text [file]                    Search a [file] for matching lines which include text.

Options:
    IGNORE_CASE                             Enable case-insensitive searching.

Examples:
    minigrep text file.txt                  Performs a search in file.txt for text.
    minigrep text file.txt > output         Performs a search in file.txt for text and stores the output in output file.
    IGNORE_CASE=1 minigrep text file.txt    Performs a case-insensitive search in file.txt for text.
```

## Potential Features

**Case Insensitivity (ADDED):** Add an option to perform case-insensitive searches. You could use a command-line argument to toggle this feature.

**Regex Support:** Integrate regular expressions to allow for more complex search patterns.

**Line Number Display:** Modify the output to show line numbers along with the matching lines.

**Search in Multiple Files:** Expand the functionality to search across multiple files or directories, rather than just a single file.

**File Type Filtering:** Add a feature to filter files by type.

**Highlight Matches:** Implement a way to highlight or emphasize matching text in the output to make it easier to spot.

**Support for Binary Files:** Add an option to handle binary files or to skip them entirely during the search.

**Performance Improvements:** Optimize the search algorithm or implement parallel processing to handle large files or directories more efficiently.

**Interactive Mode:** Create an interactive mode where users can input search queries and navigate through results more dynamically.

**Customizable Output Formatting:** Allow users to customize the format of the output, such as showing matches in a specific color or format.
