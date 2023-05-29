use clap::Parser;

/// Search for a pattern in a filepath, and display lines that contain it
#[derive(Parser)]
struct Cli {
    /// Pattern to look for
    pattern: String,

    /// Path of where to search for the pattern
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct GahError(String);

fn main() -> Result<(), GahError> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .map_err(|e| GahError(format!("Error reading `{:?}`: {}", &args.path, e)))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
