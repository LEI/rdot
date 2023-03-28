// use std::process::Output;

use std::{fs, io, path::Path};

use clap::{Args, Parser, Subcommand};
use color_eyre::{
    eyre::{Context, Result},
    Report,
};

use crate::{
    cli::command::Command,
    core::{config::Config, role::Role},
    filter,
    package::{config::PackageConfig, Action},
};

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

    /// The name of the roles configuration file.
    #[arg(short = 'C', long, default_value = "Dotfile", global = true)]
    pub role_config_name: std::path::PathBuf,

    /// Runs without applying changes.
    #[arg(short, long, global = true)]
    pub dry_run: bool,

    /// Increases logging verbosity.
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
}

impl Cli {
    //     pub fn run(self, mut config: Config, args: &Vec<String>, out: &mut Output) -> Result<()> {
    //         let matches = self.command.get_matches_from(args);

    //         Commands::from_arg_matches(matches)?.run(config, out);
    //     }
    pub fn run(self) -> Result<()> {
        color_eyre::install()?;
        if self.verbose > 0 {
            println!("CLI: {:#?}", self);
        }
        if self.dry_run {
            println!("Dry-run");
        }

        // println!("Loading global config: {:?}", config_file);
        let config = Config::load(&self.config_file).wrap_err(format!(
            "failed to load config: {}",
            &self.config_file.display()
        ))?;
        if self.verbose > 1 {
            println!("Loaded global config: {:#?}", config);
        }
        let base_dir = self.config_file.parent().unwrap();

        // TODO: resolve dependency graph

        // Get the global stdout entity and aquire a lock on it
        let stdout = io::stdout();
        let mut output = stdout.lock();

        match &self.command {
            Commands::List(args) => args.clone().run(config, &mut output)?,
            Commands::Install(args) => {
                let mut roles = filter(&config, &args.filter)?;
                let file_name = &self.role_config_name;

                run_command(
                    Action::Install,
                    &mut roles,
                    base_dir,
                    file_name,
                    self.dry_run,
                    self.verbose,
                )?
            }
            Commands::Remove(args) => {
                let mut roles = filter(&config, &args.filter)?;
                let file_name = &self.role_config_name;

                run_command(
                    Action::Remove,
                    &mut roles,
                    base_dir,
                    file_name,
                    self.dry_run,
                    self.verbose,
                )?
            }
        }

        Ok(())
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
        let role_config = PackageConfig::load(file).wrap_err("failed to load role config")?;
        if verbose > 1 {
            println!("Loaded role config: {:#?}", role_config);
        }

        role.install_or_remove(action, role_config, dry_run, verbose)
            .wrap_err(format!("failed to {}", action))?;
    }

    Ok(())
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Lists the specified roles.
    #[command(aliases = ["l", "ls"])]
    List(list::List),

    /// Installs the specified roles.
    #[command(aliases = ["i"])]
    Install(RoleArgs),

    /// Removes the specified roles.
    #[command(aliases = ["r"])]
    Remove(RoleArgs),
}

#[derive(Clone, Default, Debug, Args)]
pub struct RoleArgs {
    /// Filters the roles to act on.
    #[clap(allow_hyphen_values = false)]
    pub filter: Vec<String>,

    /// Synchronize the configured roles.
    #[clap(short, long)]
    pub sync: bool,
}
