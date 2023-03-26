use std::io::{StdoutLock, Write};

use clap::Args;

use crate::{core::config::Config, filter};

use super::{command::Command, PackageArgs};

#[derive(Debug, Default, Args)]
pub struct List {
    /// Output in an easily parseable format
    #[clap(long, visible_short_alias = 'x', conflicts_with = "json")]
    parseable: bool,

    /// Output in json format
    #[clap(long)]
    json: bool,
}

impl Command for List {
    fn run(
        self,
        args: &PackageArgs,
        config: Config,
        output: &mut StdoutLock,
    ) -> color_eyre::Result<()> {
        writeln!(output, "Available packages: {}", config.packages.len())?;
        let packages = filter(&config, args)?;

        // let pb = indicatif::ProgressBar::new(u64::try_from(packages.len())?);
        for package in packages {
            // std::thread::sleep(std::time::Duration::from_millis(1000));
            writeln!(output, "{}: {}", package.name, package.path.display())?;
            // pb.println(format!("[+] {}: {}", package.name, package.path.display()));
            // pb.inc(1);
        }
        // pb.finish_with_message("done");

        Ok(())
    }
}
