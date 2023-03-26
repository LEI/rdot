use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{eyre, Result};
use serde::Deserialize;

use crate::{cli::PackageArgs, package::Package};

pub mod package;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Environment
    pub env: Option<HashMap<String, String>>,

    /// Packages
    pub packages: HashMap<String, String>,

    /// Configuration file path
    path: Option<PathBuf>,
}

impl Config {
    pub fn load(config_file: &Path) -> Result<Self> {
        // let config = Self { env: load_env() };

        let contents = fs::read_to_string(config_file)?;
        let mut config: Config = toml::from_str(contents.as_str())?;
        config.path = Some(config_file.to_owned());

        Ok(config)
    }

    pub fn filter(&self, args: &PackageArgs) -> Result<Vec<Package>> {
        if args.filter.is_empty() {
            return Ok(self
                .packages
                .iter()
                .map(|package| package.into())
                // .map(|package| self.parse(package))
                .collect());
        }

        let mut result: Vec<Package> = vec![];

        for arg in &args.filter {
            let package = self.packages.iter().find(|package| package.0 == arg);
            if package.is_none() {
                return Err(eyre!("Invalid package in arguments: {}", arg));
            }
            result.push(package.unwrap().into());
            // let mut package: Package = self.parse(package.unwrap());
            // if package.path.is_relative() {
            //     let current_dir = dirs::CURRENT.to_path_buf();
            //     package.path = current_dir.join(&package.path);
            // }
            // result.push(package);
        }

        Ok(result)
    }
}

// fn load_env() -> HashMap<String, String> {
//     let mut env = HashMap::new();

//     env
// }
