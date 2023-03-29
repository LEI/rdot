use std::io;

use clap::Parser;
use color_eyre::eyre::Result;

use rdot::cli::Cli;

fn main() -> Result<()> {
    // TODO: disable with assert_cmd
    color_eyre::install()?;
    let cli = Cli::parse();

    let config = cli.load_config()?;

    // Get the global stdout entity and aquire a lock on it
    let stdout = io::stdout();
    let mut output = stdout.lock();

    cli.run(config, &mut output)
}
