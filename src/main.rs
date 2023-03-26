use std::{fs, io, path::Path};

use clap::Parser;
use color_eyre::{
    eyre::{Context, Result},
    Report,
};

use rdot::{
    cli::{command::Command, list::List, Cli, Commands},
    core::config::Config,
    filter,
    package::{config::PackageConfig, Action, Package},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    if cli.verbose > 0 {
        println!("CLI: {:#?}", cli);
    }
    if cli.dry_run {
        println!("Dry-run");
    }

    // println!("Loading global config: {:?}", config_file);
    let config = Config::load(&cli.config_file).wrap_err(format!(
        "failed to load config: {}",
        &cli.config_file.display()
    ))?;
    if cli.verbose > 1 {
        println!("Loaded global config: {:#?}", config);
    }
    let base_dir = cli.config_file.parent().unwrap();

    // TODO: resolve dependency graph

    // Get the global stdout entity and aquire a lock on it
    let stdout = io::stdout();
    let mut output = stdout.lock();

    match &cli.command {
        Commands::List(args) => List::default().run(args, config, &mut output)?,
        Commands::Install(args) => {
            let mut packages = filter(&config, args)?;
            let file_name = &cli.package_config_name;

            run(
                Action::Install,
                &mut packages,
                base_dir,
                file_name,
                cli.dry_run,
                cli.verbose,
            )?
        }
        Commands::Remove(args) => {
            let mut packages = filter(&config, args)?;
            let file_name = &cli.package_config_name;

            run(
                Action::Remove,
                &mut packages,
                base_dir,
                file_name,
                cli.dry_run,
                cli.verbose,
            )?
        }
    }

    Ok(())
}

fn run(
    action: Action,
    packages: &mut Vec<Package>,
    base_dir: &Path,
    file_name: &Path,
    dry_run: bool,
    verbose: u8,
) -> Result<(), Report> {
    for package in packages {
        // Path is relative to the package config where it is defined, instead of cwd
        package.path = if package.path.is_relative() {
            base_dir.join(&package.path)
        } else {
            package.path.to_path_buf()
        };
        // Note: this resolves symlinks and uses UNC on Windows
        // https://github.com/rust-lang/rust/issues/42869
        package.path = fs::canonicalize(&package.path).wrap_err(format!(
            "failed to canonicalize: {}",
            package.path.display()
        ))?;
        let file = package.path.join(file_name);

        // println!("Loading package config: {:?}", file);
        let package_config = PackageConfig::load(file).wrap_err("failed to load package config")?;
        if verbose > 1 {
            println!("Loaded package config: {:#?}", package_config);
        }

        package
            .install_or_remove(action, package_config, dry_run, verbose)
            .wrap_err(format!("failed to {}", action))?;
    }

    Ok(())
}
