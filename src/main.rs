use clap::Parser;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
    path::PathBuf,
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
    let input: Box<dyn Read> = if args.filepath == PathBuf::from("-") {
        Box::new(stdin().lock())
    } else {
        let file = File::open(&args.filepath)
            .map_err(|e| GahError(format!("Error opening `{:?}`: {}", &args.filepath, e)))?;
        Box::new(file)
    };
    let matches = match_from_reader(&args.pattern, BufReader::new(input));

    println!("{}", matches.join("\n"));
    Ok(())
}
