use std::fmt::Display;
use std::path::PathBuf;

use color_eyre::Result;

use crate::config::package::PackageConfig;
use crate::line::Line;
use crate::link::Link;
use crate::rtx::Rtx;

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
    ) -> Result<()> {
        println!("--- {:?} package: {}", action, self.name);

        let package_settings = config.settings.unwrap_or_default();
        if config.settings.is_some() && package_settings.verbose {
            println!("Package settings (verbose): {:#?}", package_settings);
        }

        if let Some(dependencies) = &config.dependencies {
            for dep in dependencies {
                println!("# TODO: import {} from {}", dep.0, dep.1);
            }
        }

        // println!("Env: {:?}", config.env);
        if let Some(env) = &config.env {
            for var in env {
                println!("# TODO: export {}={}", var.0, var.1);
            }
        }

        // println!("Lines: {:?}", config.lines);
        if let Some(lines) = &config.lines {
            for value in lines {
                let line: Line = value.into();

                line.execute(action, self.path.to_path_buf(), dry_run)?;
            }
        }

        // println!("Links: {:?}", config.links);
        if let Some(links) = &config.links {
            for value in links {
                let link: Link = value.into();

                link.execute(action, self.path.to_path_buf(), dry_run, package_settings)
                    .expect("failed to link")
            }
        }

        // println!("Tools: {:?}", config.tools);
        if let Some(tools) = &config.tools {
            for tool in tools {
                let tool = format!("{}@{}", tool.0, tool.1);
                let rtx = Rtx::new();

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
