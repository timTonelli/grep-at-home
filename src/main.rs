use anyhow::{self, Context};
use clap::Parser;
use std::{
    fs::File,
    io::{self, stdin, stdout, BufRead, BufReader, Read, StdoutLock, Write},
    path::PathBuf,
};

/// Search for a pattern in a filepath, and display lines that contain it
#[derive(Parser)]
struct Cli {
    /// Pattern to look for
    pattern: String,
    /// Path of where to search for the pattern
    filepath: Option<PathBuf>,
}

fn print_matches<R: BufRead>(needle: &str, haystack: R, mut writer: StdoutLock) -> io::Result<()> {
    haystack
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.to_lowercase().contains(&needle.to_lowercase()))
        .try_for_each(|l| writeln!(writer, "{}", l.trim()))?;
    writer.flush()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let reader: Box<dyn Read> = if let Some(path) = &args.filepath {
        let file = File::open(&path).with_context(|| format!("While opening {:?}", &path))?;
        Box::new(file)
    } else {
        Box::new(stdin().lock())
    };
    let writer = stdout().lock();
    print_matches(&args.pattern, BufReader::new(reader), writer)?;
    Ok(())
}
