use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{eyre, Context, Result};
use serde::Deserialize;

use super::{dirs, role::Role};

#[derive(Debug, Deserialize)]
pub enum Os {
    Darwin,
    Linux,
    Windows,
}

#[derive(Debug, Default, Deserialize)]
#[serde(untagged)]
pub enum OsValue {
    #[default]
    None,
    String(Os),
    Array(Vec<Os>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RoleValue {
    String(String),
    Table(Role),
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// Environment
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Operating system
    #[serde(default)]
    pub os: OsValue,

    /// Roles
    #[serde(default)]
    pub roles: HashMap<String, RoleValue>,

    /// Configuration file path
    #[serde(default)]
    pub path: PathBuf,
}

impl Config {
    pub fn load(config_file: &Path) -> Result<Self> {
        let parent_dir = config_file
            .parent()
            .expect("Config file has no parent directory");
        // Quickfix static config
        let config_file = match parent_dir.to_str().unwrap() {
            "$RDOT_CONFIG_DIR" => dirs::CONFIG
                .to_path_buf()
                .join(config_file.strip_prefix("$RDOT_CONFIG_DIR")?),
            _ => config_file.to_path_buf(),
        };

        // let config = Self { env: load_env() };

        let contents = fs::read_to_string(&config_file)
            .wrap_err(format!("failed to read: {}", &config_file.display()))?;
        let mut config: Config = toml::from_str(contents.as_str())?;
        config.path = config_file;

        Ok(config)
    }

    /// Filters the configured roles according to the provided filter in arguments.
    pub fn filter_roles(self, filter: Vec<String>) -> Result<Vec<Role>> {
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

// fn load_env() -> HashMap<String, String> {
//     let mut env = HashMap::new();

//     env
// }
