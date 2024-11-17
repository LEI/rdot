use std::io::{StdoutLock, Write};

use clap::Args;
use color_eyre::Result;

use crate::{
    cli::{commands::Command, output::Format, GlobalOptions},
    config::Config,
};

const PROGRESS_TEMPLATE: &str = "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}";

#[derive(Debug, Default, Args)]
pub(crate) struct List {
    /// Output format
    #[clap(short, long, value_enum, default_value_t = Format::Default)]
    format: Format,

    /// Filter arguments
    args: Vec<String>,
}

impl Command for List {
    fn run(self, config: &Config, _options: GlobalOptions, output: &mut StdoutLock) -> Result<()> {
        debug!("Global environment: {:#?}", config.env);
        let count = &config.get_roles().len();
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
            Format::Default => {
                writeln!(output, "Available roles: {}", count)?;

                let pb = indicatif::ProgressBar::new(u64::try_from(roles.len())?);
                pb.set_style(
                    indicatif::ProgressStyle::with_template(PROGRESS_TEMPLATE)
                        .unwrap()
                        .progress_chars("##-"),
                );
                for role in roles {
                    // writeln!(output, "{}", role)?;

                    // std::thread::sleep(std::time::Duration::from_millis(1000));

                    // pb.println(format!("[+] {}: {}", role.name, role.path.display()));
                    // "    Finished de"
                    pb.println(format!("[+] {}", role));
                    pb.inc(1);
                }
                pb.finish_with_message("done");
                // HumanDuration(Duration::from_secs(8)).to_string()
            }
        }

        Ok(())
    }
}
