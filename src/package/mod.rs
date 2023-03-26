use std::fmt::Display;
use std::path::PathBuf;

use color_eyre::Result;

use crate::package::config::SystemPackage;
use crate::package::line::Line;
use crate::package::link::Link;
use crate::rtx::Rtx;
use crate::system::System;

use self::config::PackageConfig;

pub mod config;
pub(crate) mod line;
pub(crate) mod link;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Install,
    Remove,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let action = match self {
            Self::Install => "install",
            Self::Remove => "remove",
        };
        write!(f, "{}", action)
    }
}

#[derive(Debug)]
pub struct Package {
    /// The name of the package to install
    pub name: String,

    /// The path to the package directory
    pub path: PathBuf,
}

impl Package {
    /// Lists packages.
    pub fn list(packages: &Vec<Package>, _sync: bool) -> Result<()> {
        for package in packages {
            println!("{}: {}", package.name, package.path.display());

            // if sync {
            //     let status = package.status();
            //     println!("Sync status: {:?}", status);
            // }
        }

        Ok(())
    }

    /// Installs a package with a given configuration.
    pub fn install_or_remove(
        &self,
        action: Action,
        config: PackageConfig,
        dry_run: bool,
        verbose: u8,
    ) -> Result<()> {
        println!("--- {:?} package: {}", action, self.name);

        let package_settings = config.settings;
        if verbose > 0 {
            println!("Package settings: {:#?}", package_settings);
        }

        for dep in &config.dependencies {
            println!("# TODO: import {} from {}", dep.0, dep.1);
        }

        // println!("Env: {:?}", config.env);
        for var in &config.env {
            println!("# TODO: export {}={}", var.0, var.1);
        }

        // println!("Lines: {:?}", config.lines);
        for value in &config.lines {
            let line: Line = value.into();

            line.execute(action, self.path.to_path_buf(), dry_run)?;
        }

        // println!("Links: {:?}", config.links);
        for value in &config.links {
            let link: Link = value.into();

            link.execute(action, self.path.to_path_buf(), dry_run, package_settings)?;
        }

        for pkg in &config.system {
            let sp = SystemPackage::from(pkg.1);
            let pkg = format!("{}@{:?}", pkg.0, sp.version);
            let system = System::default();

            println!("system {} {}", action, pkg);
            match action {
                Action::Install => {
                    if !dry_run {
                        system.install(&pkg);
                    }
                }
                Action::Remove => {
                    if !dry_run && !sp.keep {
                        system.remove(&pkg);
                    }
                }
            }
        }

        // println!("Tools: {:?}", config.tools);
        for tool in &config.tools {
            let tool = format!("{}@{}", tool.0, tool.1);
            let rtx = Rtx::default();

            println!("rtx {} {}", action, tool);
            match action {
                Action::Install => {
                    if !dry_run {
                        rtx.install(&tool);
                    }
                }
                Action::Remove => {
                    if !dry_run {
                        rtx.remove(&tool);
                    }
                }
            }
        }

        println!("--- Done");
        Ok(())
    }

    // fn status(&self) -> PackageStatus {
    //     let installed = false;

    //     PackageStatus {
    //         installed,
    //         path: self.path.to_path_buf(),
    //     }
    // }
}

// #[derive(Debug)]
// struct PackageStatus {
//     installed: bool,
//     path: PathBuf,
// }

impl From<(&String, &String)> for Package {
    fn from(value: (&String, &String)) -> Self {
        let name = value.0.to_string();
        let path = PathBuf::from(value.1);

        Self { name, path }
    }
}
