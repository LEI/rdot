use std::io::{StdoutLock, Write};

use clap::Args;
use color_eyre::eyre::Result;

use crate::{
    cli::{commands::Command, GlobalOptions},
    config::{
        init::{init_dir, InitDir},
        Config,
    },
};

#[derive(Debug, Default, Args)]
pub(crate) struct Init {
    /// Override existing configuration.
    #[arg(short, long)]
    pub(crate) force: bool,
}

impl Command for Init {
    fn run(self, _config: &Config, options: GlobalOptions, output: &mut StdoutLock) -> Result<()> {
        let file = options.config_file;
        let created = init_dir(
            &file,
            InitDir {
                confirm: !options.yes,
                force: self.force,
                dry_run: options.dry_run,
            },
        )?
        .is_some();
        if created {
            writeln!(output, "Created: {}", file.display())?;
        }

        Ok(())
    }
}
