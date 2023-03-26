use clap::{Args, Parser, Subcommand};

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

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Lists the specified packages.
    #[command(aliases = ["l", "ls"])]
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

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    fn bin() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

    #[test]
    fn test_help_command() {
        bin()
            .arg("--dry-run")
            .assert()
            .failure()
            .code(2)
            .stderr(predicate::str::contains("Usage:"));
    }

    #[test]
    fn test_list_command() {
        bin()
            .arg("list")
            .arg("--dry-run")
            .arg("--config-file=examples/config.toml")
            .assert()
            .success()
            .stdout(predicate::str::contains("Available"));
    }

    #[test]
    fn test_install_command() {
        bin()
            .arg("install")
            .arg("--dry-run")
            .arg("--config-file=examples/config.toml")
            .assert()
            .success()
            .stdout(predicate::str::contains("Dry-run"));
    }

    #[test]
    fn test_remove_command() {
        bin()
            .arg("remove")
            .arg("--dry-run")
            .arg("--config-file=examples/config.toml")
            .assert()
            .success()
            .stdout(predicate::str::contains("Dry-run"));
    }
}
