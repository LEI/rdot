use std::io;

use color_eyre::eyre::Result;

use rdot::cli::Cli;

fn main() -> Result<()> {
    color_eyre::install()?;
    // #[cfg(debug_assertions)]
    env_logger::init();
    let (cli, config) = Cli::init()?;

    // Get the global stdout entity and aquire a lock on it
    let stdout = io::stdout();
    let mut output = stdout.lock();

    cli.run(&config, &mut output)
}
