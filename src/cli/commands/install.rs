use std::io::StdoutLock;

use clap::Args;
use color_eyre::Result;

use crate::{
    cli::{commands::Command, options::RoleOptions, run_command, GlobalOptions},
    config::Config,
    role::action::Action,
};

#[derive(Clone, Debug, Default, Args)]
pub(crate) struct Install {
    /// Filter arguments
    args: Vec<String>,

    #[clap(flatten)]
    options: RoleOptions,
}

impl Command for Install {
    fn run(self, config: &Config, options: GlobalOptions, _output: &mut StdoutLock) -> Result<()> {
        let mut roles = config.filter_roles(self.args)?;
        let file_name = self.options.role_config_name;
        let config_file = options.config_file;
        let base_dir = config_file.parent().unwrap();

        run_command(
            Action::Install,
            &mut roles,
            base_dir,
            &file_name,
            options.dry_run,
            options.verbose,
        )
    }
}
