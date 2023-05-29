use clap::Parser;

/// Search for a pattern in a filepath, and display lines that contain it
#[derive(Parser)]
struct Cli {
    /// Pattern to look for
    #[arg(long)]
    pattern: String,

    /// Path of where to search for the pattern
    #[arg(long)]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
