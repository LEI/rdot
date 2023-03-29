use std::io::StdoutLock;

use clap::Args;
use color_eyre::Result;

use crate::{
    cli::{commands::Command, run_command, GlobalOptions},
    config::Config,
    role::action::Action,
};

#[derive(Clone, Debug, Default, Args)]
pub(crate) struct Remove {
    /// Filter arguments
    args: Vec<String>,
}

impl Command for Remove {
    fn run(self, config: Config, options: GlobalOptions, _output: &mut StdoutLock) -> Result<()> {
        let mut roles = config.filter_roles(self.args)?;
        let file_name = options.role_config_name;
        let base_dir = options.config_file.parent().unwrap();

        run_command(
            Action::Remove,
            &mut roles,
            base_dir,
            &file_name,
            options.dry_run,
            options.verbose,
        )
    }
}
