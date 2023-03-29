use std::io::StdoutLock;

use clap::Args;
use color_eyre::Result;

use crate::{
    cli::{commands::Command, run_command, GlobalOptions},
    config::Config,
    role::action::Action,
};

#[derive(Clone, Debug, Default, Args)]
pub(crate) struct Install {
    /// Filter arguments
    args: Vec<String>,
}

impl Command for Install {
    fn run(self, config: Config, options: GlobalOptions, _output: &mut StdoutLock) -> Result<()> {
        println!("--- {:#?}", self.args);
        let mut roles = config.filter_roles(self.args)?;
        println!("--- {:#?}", options);
        let file_name = options.role_config_name;
        let base_dir = options.config_file.parent().unwrap();

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
