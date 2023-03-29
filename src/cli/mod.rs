use std::{
    fs,
    io::StdoutLock,
    path::{Path, PathBuf},
};

use clap::{ArgAction, Parser, Subcommand};
use color_eyre::{
    eyre::{Context, Result},
    Report,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::{load_toml, Config},
    role::action::{Action, Role},
};

use self::commands::{install, list, remove, Command};

pub mod commands;
pub(crate) mod output;

// TODO: use src/dirs.rs
const DEFAULT_CONFIG_FILE: &str = "$RDOT_CONFIG_DIR/config.toml";

/// Global CLI configuration.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,

    #[clap(flatten)]
    global_options: GlobalOptions,
}

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

    /// The name of the roles configuration file.
    #[arg(
        short = 'C',
        long = "config-name",
        default_value = "Dotfile",
        global = true
    )]
    // pub(crate) config_name: PathBuf,
    pub role_config_name: PathBuf,

    /// Runs without applying changes.
    #[arg(short, long, global = true)]
    pub(crate) dry_run: bool,

    /// Increases logging verbosity.
    #[arg(short, long, action = ArgAction::Count, global = true)]
    pub(crate) verbose: u8,
}

impl Cli {
    /// Load global configuration
    pub fn load_config(&self) -> Result<Config> {
        let config_file = &self.global_options.config_file;
        // println!("Loading global config: {:?}", &config_file);

        Config::load(config_file)
            .wrap_err(format!("Failed to load config: {}", config_file.display()))
    }
}

// FIXME: impl Command for Cli requires global_options
impl Cli {
    pub fn run(self, config: Config, stdout: &mut StdoutLock) -> Result<()> {
        if self.global_options.verbose > 0 {
            println!("CLI: {:#?}", self);
        }
        let options = self.global_options;
        if options.dry_run {
            println!("Dry-run");
        }
        if options.verbose > 1 {
            println!("Loaded global config: {:#?}", config);
        }

        // TODO: resolve dependency graph

        match self.command {
            Commands::List(list) => list.run(config, options, stdout),
            Commands::Install(install) => install.run(config, options, stdout),
            Commands::Remove(remove) => remove.run(config, options, stdout),
        }
    }
}

fn run_command(
    action: Action,
    roles: &mut Vec<Role>,
    base_dir: &Path,
    file_name: &Path,
    dry_run: bool,
    verbose: u8,
) -> Result<(), Report> {
    for role in roles {
        // Path is relative to the role config where it is defined, instead of cwd
        role.path = if role.path.is_relative() {
            base_dir.join(&role.path)
        } else {
            role.path.to_path_buf()
        };
        // Note: this resolves symlinks and uses UNC on Windows
        // https://github.com/rust-lang/rust/issues/42869
        role.path = fs::canonicalize(&role.path)
            .wrap_err(format!("failed to canonicalize: {}", role.path.display()))?;
        let file = role.path.join(file_name);

        // println!("Loading role config: {:?}", file);
        let role_config = load_toml(&file).wrap_err("failed to load role config")?;
        if verbose > 1 {
            println!("Loaded role config: {:#?}", role_config);
        }

        role.install_or_remove(action, role_config, dry_run, verbose)
            .wrap_err(format!("failed to {}", action))?;
    }

    Ok(())
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Lists the specified roles.
    #[command(aliases = ["l", "ls"])]
    List(list::List),

    /// Installs the specified roles.
    #[command(aliases = ["i"])]
    Install(install::Install),

    /// Removes the specified roles.
    #[command(aliases = ["r"])]
    Remove(remove::Remove),
}
