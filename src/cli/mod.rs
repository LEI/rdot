use std::{
    fs,
    io::StdoutLock,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use color_eyre::{
    eyre::{eyre, Context, Result},
    Report,
};

use crate::{
    config::{loader::load_toml, Config},
    role::action::{Action, Role},
};

use self::{
    commands::{init, install, list, remove, Command},
    options::GlobalOptions,
};

pub mod commands;
mod options;
pub(crate) mod output;

/// Rdot
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,

    #[clap(flatten)]
    global_options: GlobalOptions,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Create the global configuration file.
    Init(init::Init),

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

impl Cli {
    /// Load global configuration
    pub fn load_config(&self) -> Result<Config> {
        let config_file = &self.global_options.config_file;

        if let Commands::Init(args) = &self.command {
            if !args.force && PathBuf::from(config_file).exists() {
                return Err(eyre!(
                    "Global config already exists: {}",
                    config_file.display()
                ));
            }

            return Ok(Config::default());
        }

        Config::load(&PathBuf::from(config_file)).wrap_err(format!(
            "Failed to load global config: {}",
            config_file.display()
        ))
    }
}

// FIXME: impl Command for Cli requires global_options
impl Cli {
    pub fn run(self, config: &Config, stdout: &mut StdoutLock) -> Result<()> {
        if self.global_options.verbose > 0 {
            println!("CLI: {:#?}", self);
        }
        let options = self.global_options;
        if options.dry_run {
            println!("Dry-run");
        }

        // if options.verbose > 1 {
        //     println!("Loaded global config: {:#?}", config);
        // }

        // log::debug!("OS: {:?}", config.os);
        // let os: Vec<Os> = config.os.into();

        // TODO: resolve dependency graph
        // for (name, role) in &config.roles {
        //     let role: Role = role.into();
        //     println!("dependencies {} {:#?}", name, role.name);
        // }

        // let config = match &self.command {
        //     Commands::Init(args) => self.load_config().unwrap_or_default(),
        //     _ => self.load_config()?,
        // };
        match self.command {
            Commands::Init(args) => args.run(config, options, stdout),
            Commands::List(args) => args.run(config, options, stdout),
            Commands::Install(args) => args.run(config, options, stdout),
            Commands::Remove(args) => args.run(config, options, stdout),
        }
    }
}

// TODO: trait?
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
