use std::path::PathBuf;

use clap::{ArgAction, Parser};
use serde::{Deserialize, Serialize};

// TODO: shellexpand usage?
// use crate::config::dirs;

#[derive(Clone, Debug, Default, Parser, Serialize, Deserialize)]
pub struct Options {
    /// The full path to the configuration directory.
    #[arg(
        default_value = "$RDOT_CONFIG_DIR",
        env = "RDOT_CONFIG_DIR",
        global = true,
        help_heading = "Configuration",
        hide = true,
        // hide_default_value = true, // Default value is already shown by `env`
        long = "config",
        // next_line_help = true,
        short
    )]
    pub(crate) config_dir: PathBuf,

    /// Prints actions without applying changes.
    #[arg(short, long, global = true)]
    pub(crate) dry_run: bool,

    /// Increases logging verbosity.
    #[arg(short, long, action = ArgAction::Count, global = true)]
    pub(crate) verbose: u8,

    /// Accepts all interactive prompts.
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
    /// The name of the role configuration file.
    #[arg(
        default_value = "Dotfile", // role.toml?
        global = true,
        help_heading = "Configuration",
        long
    )]
    // pub(crate) config_name: PathBuf,
    pub role_config_name: PathBuf,
}
