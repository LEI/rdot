use std::path::PathBuf;

use clap::{ArgAction, Parser};
use serde::{Deserialize, Serialize};

// use crate::config::dirs;

const DEFAULT_CONFIG_FILE: &str = "$RDOT_CONFIG_DIR/config.toml";

#[derive(Clone, Debug, Default, Parser, Serialize, Deserialize)]
pub struct GlobalOptions {
    /// The path to the global configuration file.
    #[arg(
        short,
        long,
        default_value = DEFAULT_CONFIG_FILE,
        global = true
    )]
    pub(crate) config_file: PathBuf,

    /// Runs without applying changes.
    #[arg(short, long, global = true)]
    pub(crate) dry_run: bool,

    /// Increases logging verbosity.
    #[arg(short, long, action = ArgAction::Count, global = true)]
    pub(crate) verbose: u8,

    /// Disables all interactive prompts.
    #[arg(short, long, global = true)]
    pub(crate) yes: bool,
}

// #[derive(Parser, Serialize, Deserialize)]
// pub struct ActionOptions {
//     // #[serde(flatten)]
//     role_config: RoleConfigOptions,
// }

#[derive(Clone, Debug, Default, Parser, Serialize, Deserialize)]
pub(crate) struct RoleOptions {
    /// The name of the roles configuration file.
    #[arg(
        short = 'C',
        long = "config-name",
        default_value = "Dotfile",
        global = true
    )]
    // pub(crate) config_name: PathBuf,
    pub role_config_name: PathBuf,
}
