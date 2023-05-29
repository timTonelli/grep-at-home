use clap::Parser;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
};

/// Search for a pattern in a filepath, and display lines that contain it
#[derive(Parser)]
struct Cli {
    /// Pattern to look for
    pattern: String,

    /// Path of where to search for the pattern
    filepath: Option<std::path::PathBuf>,
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
    let input: Box<dyn Read> = if let Some(path) = &args.filepath {
        let file = File::open(&path)
            .map_err(|e| GahError(format!("Error opening `{:?}`: {}", &args.filepath, e)))?;
        Box::new(file)
    } else {
        Box::new(stdin().lock())
    };
    let matches = match_from_reader(&args.pattern, BufReader::new(input));

    println!("{}", matches.join("\n"));
    Ok(())
}
