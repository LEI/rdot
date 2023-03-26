use std::{fs, path::Path};

#[macro_use]
extern crate lazy_static;

use clap::Parser;
use color_eyre::{
    eyre::{Context, Result},
    Report,
};

use crate::{
    cli::{Cli, Commands},
    config::{package::PackageConfig, Config},
    package::{Action, Package},
};

mod cli;
mod config;
mod dirs;
mod env;
mod line;
mod link;
mod package;
mod rtx;

// Action: install/remove
// Directory: source -> target
// Strategy: local, ansible
// Tools: rtx, brew

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    if cli.verbose > 0 {
        println!("CLI: {:#?}", cli);
    }
    if cli.dry_run {
        println!("Dry-run");
    }

    let config_file = cli.config_file;
    let parent_dir = config_file
        .parent()
        .expect("Config file has no parent directory");
    // Quickfix static config
    let config_file = match parent_dir.to_str().unwrap() {
        "$RDOT_CONFIG_DIR" => {
            let config_directory = dirs::CONFIG.to_path_buf();
            let config_file = config_directory.join(
                config_file
                    .strip_prefix("$RDOT_CONFIG_DIR")
                    .expect("failed to strip prefix"),
            );

            config_file
        }
        _ => config_file.to_owned(),
    };
    println!("Loading global config: {:?}", config_file);
    let config = Config::load(&config_file).expect("failed to load config");
    println!("Loaded global config: {:?}", config);

    // let log_level = *env::RTX_LOG_LEVEL;
    // logger::init(log_level, *env::RTX_LOG_FILE_LEVEL);
    // handle_ctrlc();

    // match run(&args).with_section(|| VERSION.to_string().header("Version:")) {
    //     Ok(()) => Ok(()),
    //     // Err(err) if log_level < log::LevelFilter::Debug => {
    //     //     display_friendly_err(err);
    //     //     exit(1);
    //     // },
    //     Err(err) => Err(err).suggestion("Run with RTX_DEBUG=1 for more information."),
    // }

    // Package configuration file name (default: Dotfile)
    let file_name = &cli.package_config_name;

    // TODO: resolve dependency graph

    match &cli.command {
        Commands::List(args) => {
            let packages = config.filter(args)?;

            println!("Available packages: {}", config.packages.len());
            Package::list(&packages, args.sync)?;
        }
        Commands::Install(args) => {
            let mut packages = config.filter(args)?;

            run(Action::Install, &mut packages, file_name, cli.dry_run)?
        }
        Commands::Remove(args) => {
            let mut packages = config.filter(args)?;

            run(Action::Remove, &mut packages, file_name, cli.dry_run)?
        }
    }

    Ok(())
}

fn run(
    action: Action,
    packages: &mut Vec<Package>,
    file_name: &Path,
    dry_run: bool,
) -> Result<(), Report> {
    for package in packages {
        // TODO: path relative to package config instead of cwd
        // Note: this resolves symlinks and uses UNC on Windows
        // https://github.com/rust-lang/rust/issues/42869
        package.path = fs::canonicalize(&package.path).wrap_err("failed to canonicalize")?;
        let file = package.path.join(file_name);

        println!("Loading package config: {:?}", file);
        let package_config = PackageConfig::load(file).wrap_err("failed to load package config")?;
        println!("Loaded package config: {:?}", package_config);

        package
            .install_or_remove(action, package_config, dry_run)
            .wrap_err(format!("failed to {}", action))?;
    }

    Ok(())
}

// fn run(args: &Vec<String>) -> Result<()> {
//     // let out = &mut Output::new();

//     // // show version before loading config in case of error
//     // cli::version::print_version_if_requested(&env::ARGS, out);

//     let config = Config::load()?;
//     println!("Config: {:#?}", config);
//     // let config = shims::handle_shim(config, args, out)?;
//     // if config.should_exit_early {
//     //     return Ok(());
//     // }
//     // let cli = Cli::new_with_external_commands(&config);
//     // cli.run(config, args, out)
//     Ok(())
// }
