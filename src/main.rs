use clap::Parser;

/// Search for a pattern in a filepath, and display lines that contain it
#[derive(Parser)]
struct Cli {
    /// Pattern to look for
    pattern: String,

    /// Path of where to search for the pattern
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => content,
        Err(error) => return Err(error.into()),
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
