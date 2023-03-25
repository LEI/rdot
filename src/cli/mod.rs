use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
pub struct Cli {
    /// The pattern to look for
    #[arg(default_value = "*")]
    pattern: String,

    /// The path to the file to read
    #[arg(default_value = "Cargo.toml")]
    path: std::path::PathBuf,

    /// The output path
    #[arg(short = 'o', long = "output", default_value = "~")]
    output: std::path::PathBuf,
}
