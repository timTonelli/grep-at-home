use clap::Parser;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

/// Search for a pattern in a filepath, and display lines that contain it
#[derive(Parser)]
struct Cli {
    /// Pattern to look for
    pattern: String,

    /// Path of where to search for the pattern
    filepath: std::path::PathBuf,
}

#[derive(Debug)]
struct GahError(String);

fn match_from_reader<T: BufRead>(needle: &str, haystack: T) -> Vec<String> {
    haystack
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.contains(needle))
        .collect()
}

/*
* TODO: Look into the `anyhow` crate for more in-depth error handling
* https://rust-cli.github.io/book/tutorial/errors.html
*/
fn main() -> Result<(), GahError> {
    let args = Cli::parse();
    let matches;
    if args.filepath == std::path::PathBuf::from("-") {
        let stdin_reader = BufReader::new(stdin().lock());
        matches = match_from_reader(&args.pattern, stdin_reader);
    } else {
        let file_reader = BufReader::new(File::open(&args.filepath).unwrap());
        matches = match_from_reader(&args.pattern, file_reader);
    }

    println!("{}", matches.join("\n"));
    Ok(())
}
