use std::io::{StdoutLock, Write};

use clap::Args;
use color_eyre::Result;

use crate::{
    cli::{commands::Command, output::Format, GlobalOptions},
    config::Config,
};

#[derive(Debug, Default, Args)]
pub(crate) struct List {
    /// Output format
    #[clap(short, long, value_enum, default_value_t = Format::None)]
    format: Format,

    /// Filter arguments
    args: Vec<String>,
}

impl Command for List {
    fn run(self, config: Config, _options: GlobalOptions, output: &mut StdoutLock) -> Result<()> {
        let count = &config.roles.len();
        let roles = config.filter_roles(self.args)?;

        match self.format {
            Format::Json => {
                let json = serde_json::to_string_pretty(&roles)?;
                writeln!(output, "{}", json)?;
            }
            Format::Raw => {
                let lines: Vec<String> = roles
                    .iter()
                    .map(|role| format!("{} {}\n", role.name, role.path.display()))
                    .collect();
                write!(output, "{}", lines.join(""))?;
            }
            Format::None => {
                writeln!(output, "Available roles: {}", count)?;

                // let pb = indicatif::ProgressBar::new(u64::try_from(roles.len())?);
                for role in roles {
                    // std::thread::sleep(std::time::Duration::from_millis(1000));
                    writeln!(output, "{}", role)?;
                    // pb.println(format!("[+] {}: {}", role.name, role.path.display()));
                    // pb.inc(1);
                }
                // pb.finish_with_message("done");
            }
        }

        Ok(())
    }
}
