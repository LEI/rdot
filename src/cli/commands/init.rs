use std::io::{StdoutLock, Write};

use clap::Args;
use color_eyre::eyre::Result;

use crate::{
    cli::{options::GlobalOptions, progress::show},
    config::{
        init::{init_dir, InitDirOptions},
        Config,
    },
};

use super::Command;

#[derive(Debug, Default, Args)]
pub(crate) struct Init {
    /// Overrides existing configuration and schema if they exist.
    #[arg(short, long)]
    pub(crate) force: bool,
}

impl Command for Init {
    fn run(self, config: &Config, options: GlobalOptions, output: &mut StdoutLock) -> Result<()> {
        let created = init_dir(
            &options.config_dir,
            InitDirOptions {
                confirm: !options.yes,
                #[cfg(feature = "json-schema")]
                force: self.force,
                dry_run: options.dry_run,
            },
        )?;

        if created.is_some() {
            show(u64::try_from(1)?, "Created");
            writeln!(output, "Created: {}", config.path.display())?;
        }

        Ok(())
    }
}
