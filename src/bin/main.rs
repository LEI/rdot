use std::io;

use color_eyre::eyre::Result;

use rdot::cli::Cli;

fn main() -> Result<()> {
    // Disable SpanTrace capture by default
    if std::env::var("RUST_SPANTRACE").is_err() {
        std::env::set_var("RUST_SPANTRACE", "0");
    }

    color_eyre::install()?;

    // #[cfg(debug_assertions)]
    env_logger::init();

    let cli = Cli::init()?;

    // Get the global stdout entity and aquire a lock on it
    let stdout = io::stdout();
    let mut output = stdout.lock();

    cli.run(&mut output)
}
