use clap::Parser;
use color_eyre::eyre::Result;

use rdot::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.run()
}
