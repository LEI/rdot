use std::io::StdoutLock;

use clap::Args;
use color_eyre::Result;

use crate::{
    cli::{
        options::{Options, RoleOptions},
        run_command,
    },
    config::Config,
    role::action::Action,
};

use super::Command;

#[derive(Debug, Default, Args)]
pub(crate) struct Install {
    /// Filter arguments
    args: Vec<String>,

    #[clap(flatten)]
    options: RoleOptions,
}

impl Command for Install {
    fn run(self, config: &Config, options: Options, _output: &mut StdoutLock) -> Result<()> {
        let mut roles = config.filter_roles(self.args)?;
        let file_name = self.options.role_config_name;

        run_command(
            &Action::Install,
            &mut roles,
            &options.config_dir,
            &file_name,
            options.dry_run,
            options.verbose,
        )
    }
}
