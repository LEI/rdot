use std::io::StdoutLock;

use clap::Args;
use color_eyre::Result;

use crate::{
    cli::{commands::Command, options::RoleOptions, run_command, Options},
    config::Config,
    role::action::Action,
};

#[derive(Debug, Default, Args)]
pub(crate) struct Remove {
    /// Filter arguments
    args: Vec<String>,

    #[clap(flatten)]
    options: RoleOptions,
}

impl Command for Remove {
    fn run(self, config: &Config, options: Options, _output: &mut StdoutLock) -> Result<()> {
        let mut roles = config.filter_roles(self.args)?;
        let file_name = self.options.role_config_name;

        run_command(
            &Action::Remove,
            &mut roles,
            &options.config_dir,
            &file_name,
            options.dry_run,
            options.verbose,
        )
    }
}
