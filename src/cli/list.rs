use std::io::{StdoutLock, Write};

use clap::{Args, ValueEnum};

use crate::core::config::Config;

use super::{command::Command, RoleArgs};

#[derive(Clone, Default, Debug, ValueEnum)]
pub enum OutputFormat {
    /// Output in an easily parseable format
    Raw,

    /// Output in json format
    Json,

    // #[clap(skip)]
    #[default]
    None,
}

#[derive(Clone, Debug, Default, Args)]
pub struct List {
    /// Output format
    #[clap(short, long, value_enum, default_value_t = OutputFormat::None)]
    format: OutputFormat,

    /// Common arguments
    #[clap(flatten)]
    args: RoleArgs,
}

impl Command for List {
    fn run(self, config: Config, output: &mut StdoutLock) -> color_eyre::Result<()> {
        let count = &config.roles.len();
        let roles = config.filter_roles(self.args.filter)?;

        match self.format {
            OutputFormat::Json => {
                let json = serde_json::to_string_pretty(&roles)?;
                writeln!(output, "{}", json)?;
            }
            OutputFormat::Raw => {
                let lines: Vec<String> = roles
                    .iter()
                    .map(|role| format!("{} {}\n", role.name, role.path.display()))
                    .collect();
                write!(output, "{}", lines.join(""))?;
            }
            OutputFormat::None => {
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

impl From<&RoleArgs> for List {
    fn from(value: &RoleArgs) -> Self {
        Self {
            args: value.to_owned(),
            ..Default::default()
        }
    }
}
