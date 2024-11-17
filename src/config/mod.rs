use std::{
    collections::HashMap,
    env::set_var,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{eyre, Result};

#[cfg(feature = "json-schema")]
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

use crate::role::action::Role;

use self::loader::load_toml;

pub(crate) mod dirs;
mod env;
pub(crate) mod init;
pub(crate) mod loader;
pub(crate) mod os;

#[cfg(feature = "json-schema")]
const SCHEMA_NAME: &str = "schema.json";

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[serde(untagged)]
pub(crate) enum RoleValue {
    String(String),
    Table(Role),
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Environment variables.
    #[serde(default)]
    pub(crate) env: Option<HashMap<String, String>>,

    // /// Supported operating systems.
    // #[serde(default)]
    // pub(crate) os: Vec<Os>,
    /// Roles
    // #[serde(default)]
    // pub(crate) roles: RoleMap,
    pub(crate) roles: HashMap<String, RoleValue>,

    /// Resolved configuration file path.
    #[serde(skip)]
    pub(crate) path: PathBuf,
    // #[serde(default)]
    // pub(crate) options: GlobalOptions,
}

impl Config {
    #[cfg(feature = "json-schema")]
    pub(crate) fn get_schema() -> schemars::schema::RootSchema {
        let schema = schema_for!(Config);

        schema
    }

    #[cfg(feature = "json-schema")]
    pub(crate) fn get_schema_json() -> Result<String> {
        let schema = Self::get_schema();
        let json = serde_json::to_string_pretty(&schema)?;

        Ok(json)
    }

    pub(crate) fn get_default_toml() -> Result<String> {
        let config = Self::default();
        let toml = toml::to_string(&config)?;

        Ok(toml)
    }

    pub(crate) fn load(config_file: &Path) -> Result<Self> {
        let config_file = expand(config_file)?;

        let mut config: Self = load_toml(&config_file)?;
        config.path = config_file;
        debug!("set config path from load {}", config.path.display());

        config.update_env();

        Ok(config)
    }

    /// Updates environment variables.
    pub(crate) fn update_env(&self) {
        if let Some(env) = &self.env {
            for (key, value) in env {
                println!("export {}={:?}", key, value);
                set_var(key, value);
            }
        }
    }

    /// Lists all available roles.
    pub(crate) fn get_roles(&self) -> Vec<Role> {
        self.roles.iter().map(|role| role.into()).collect()
    }

    /// Filters the configured roles according to the provided filter in arguments.
    pub(crate) fn filter_roles(&self, filter: Vec<String>) -> Result<Vec<Role>> {
        if filter.is_empty() {
            return Ok(self.get_roles());
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

pub(crate) fn expand(file: &Path) -> Result<PathBuf> {
    let config_file = shellexpand::full(&file.to_string_lossy())?
        .to_string()
        .into();

    Ok(config_file)
}
