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

fn match_from_buffer<T: BufRead>(needle: &str, haystack: T) -> Vec<String> {
    let mut v = Vec::new();
    for line in haystack.lines() {
        let l = line.unwrap();
        if l.contains(needle) {
            v.push(l);
        }
    }
    return v;
}

/*
* TODO: Look into the `anyhow` crate for more in-depth error handling
* https://rust-cli.github.io/book/tutorial/errors.html
*/
fn main() -> Result<(), GahError> {
    let args = Cli::parse();
    let matches;
    if args.filepath == std::path::PathBuf::from("-") {
        let input = BufReader::new(stdin().lock());
        matches = match_from_buffer(&args.pattern, input);
    } else {
        let file = BufReader::new(File::open(&args.filepath).unwrap());
        matches = match_from_buffer(&args.pattern, file);
    }

    println!("{}", matches.join("\n"));
    Ok(())
}
