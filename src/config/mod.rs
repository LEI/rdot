use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::role::action::Role;

pub(crate) mod dirs;
mod env;

#[derive(Debug, Deserialize)]
pub(crate) enum Os {
    Darwin,
    Linux,
    Windows,
}

#[derive(Debug, Default, Deserialize)]
#[serde(untagged)]
pub(crate) enum OsValue {
    #[default]
    None,
    String(Os),
    Array(Vec<Os>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum RoleValue {
    String(String),
    Table(Role),
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// Environment
    #[serde(default)]
    pub(crate) env: HashMap<String, String>,

    /// Operating system
    #[serde(default)]
    pub(crate) os: OsValue,

    /// Roles
    #[serde(default)]
    pub(crate) roles: HashMap<String, RoleValue>,

    /// Configuration file path
    #[serde(skip)]
    pub(crate) path: PathBuf,
    // #[serde(default)]
    // pub(crate) options: GlobalOptions,
}

impl Config {
    pub(crate) fn load(config_file: &Path) -> Result<Self> {
        let parent_dir = config_file
            .parent()
            .wrap_err("Failed to get config file parent directory")?;
        // Quickfix static config
        let config_file = match parent_dir.to_str().unwrap() {
            "$RDOT_CONFIG_DIR" => dirs::CONFIG
                .to_path_buf()
                .join(config_file.strip_prefix("$RDOT_CONFIG_DIR")?),
            _ => config_file.to_path_buf(),
        };

        // let config = Self { env: load_env() };

        let mut config: Self = load_toml(&config_file)?;
        config.path = config_file;

        Ok(config)
    }

    /// Filters the configured roles according to the provided filter in arguments.
    pub(crate) fn filter_roles(self, filter: Vec<String>) -> Result<Vec<Role>> {
        if filter.is_empty() {
            return Ok(self.roles.iter().map(|role| role.into()).collect());
        }

        let mut result: Vec<Role> = vec![];

        for arg in &filter {
            let role = self.roles.iter().find(|role| role.0 == arg);
            if role.is_none() {
                return Err(eyre!("Invalid role name in arguments: {}", arg));
            }
            result.push(role.unwrap().into());
        }

        Ok(result)
    }
}

pub(crate) fn load_toml<T>(file: &PathBuf) -> Result<T>
where
    T: DeserializeOwned,
{
    let contents =
        fs::read_to_string(file).wrap_err(format!("Failed to read: {}", &file.display()))?;
    toml::from_str(contents.as_str()).wrap_err(format!("Failed to load: {}", &file.display()))
}

// fn load_env() -> HashMap<String, String> {
//     let mut env = HashMap::new();

//     env
// }
