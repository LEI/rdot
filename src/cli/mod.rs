// use std::process::Output;

use clap::{Args, Parser, Subcommand};
// use color_eyre::Result;

// use crate::core::config::Config;

pub mod command;
pub mod list;

// TODO: use src/dirs.rs
const DEFAULT_CONFIG_FILE: &str = "$RDOT_CONFIG_DIR/config.toml";

/// Global CLI configuration.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// The path to the global configuration file.
    #[arg(
        short,
        long,
        default_value = DEFAULT_CONFIG_FILE,
        global = true
    )]
    pub config_file: std::path::PathBuf,

    /// The name of the packages configuration file.
    #[arg(short = 'C', long, default_value = "Dotfile", global = true)]
    pub package_config_name: std::path::PathBuf,

    /// Runs without applying changes.
    #[arg(short, long, global = true)]
    pub dry_run: bool,

    /// Increases logging verbosity.
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
}

// impl Cli {
//     pub fn run(self, mut config: Config, args: &Vec<String>, out: &mut Output) -> Result<()> {
//         let matches = self.command.get_matches_from(args);

//         Commands::from_arg_matches(matches)?.run(config, out);
//     }
// }

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Lists the specified packages.
    #[command(aliases = ["l", "ls"])]
    // TODO: List(list::List),
    List(PackageArgs),

    /// Installs the specified packages.
    #[command(aliases = ["i"])]
    Install(PackageArgs),

    /// Removes the specified packages.
    #[command(aliases = ["r"])]
    Remove(PackageArgs),
}

#[derive(Debug, Args)]
pub struct PackageArgs {
    /// Filters the packages to act on.
    #[clap(allow_hyphen_values = false)]
    pub filter: Vec<String>,

    /// Synchronize the configured packages.
    #[clap(short, long)]
    pub sync: bool,
}
