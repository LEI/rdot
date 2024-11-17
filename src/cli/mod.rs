use std::{
    fs,
    io::{StdoutLock, Write},
    path::Path,
};

use clap::{Parser, Subcommand};
use color_eyre::{
    eyre::{eyre, Context, Result},
    Report,
};

use crate::{
    config::{expand, init::init_env, loader::load_toml, Config},
    role::action::{Action, Role},
};

use self::{
    commands::{init, install, list, remove, Command},
    options::GlobalOptions,
};

pub mod commands;
mod options;
pub(crate) mod output;
pub(crate) mod progress;
pub(crate) mod prompt;

/// Rdot
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, rename_all = "kebab-case")]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,

    #[clap(flatten)]
    global_options: GlobalOptions,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Creates the global configuration and schema file.
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
    /// Setup environment, parse command arguments and load configuration.
    pub fn init() -> Result<(Self, Config)> {
        init_env();

        let cli = Self::parse();
        let config = cli.load_config()?;

        Ok((cli, config))
    }

    /// Load the global configuration.
    pub fn load_config(&self) -> Result<Config> {
        let config_file = expand(&self.global_options.config_dir.join("config.toml"))?;

        if let Commands::Init(args) = &self.command {
            // Configuration file should not exist when running the init command
            if !args.force && config_file.exists() {
                return Err(eyre!(
                    "Global config already exists: {}",
                    config_file.display()
                ));
            }

            let config = Config {
                path: config_file,
                ..Default::default()
            };

            return Ok(config);
        }

        let mut config = Config::load(&config_file).wrap_err(format!(
            "Failed to load global config: {}",
            config_file.display()
        ))?;

        config.path = config_file;

        Ok(config)
    }
}

impl Cli {
    pub fn run(self, config: &Config, stdout: &mut StdoutLock) -> Result<()> {
        if self.global_options.verbose > 0 {
            debug!("{:#?}", self);
        }
        let options = self.global_options;
        if options.verbose > 1 {
            debug!("Loaded global config: {:#?}", config);
        }
        if options.dry_run {
            writeln!(stdout, "Dry-run")?;
        }

        // debug!("OS: {:?}", config.os);
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
            .wrap_err(format!("Failed to resolve path: {}", role.path.display()))?;
        let file = role.path.join(file_name);

        debug!("Loading role config: {}", file.display());
        let role_config = load_toml(&file).wrap_err("Failed to load role config")?;
        if verbose > 1 {
            println!("Loaded role config: {:#?}", role_config);
        }

        role.install_or_remove(action, role_config, dry_run, verbose)
            .wrap_err(format!("failed to {}", action))?;
    }

    Ok(())
}
