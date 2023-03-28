use std::{collections::HashMap, fs, path::PathBuf};

use color_eyre::eyre::{Context, Result};
use serde::Deserialize;

use super::system::SystemPackageValue;

#[derive(Debug, Default, Deserialize)]
pub struct RoleConfig {
    /// Dependencies
    #[serde(default)]
    pub dependencies: HashMap<String, String>,

    /// Environment
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Lines
    #[serde(default)]
    pub lines: HashMap<String, String>,

    /// Links
    #[serde(default)]
    pub links: HashMap<String, String>,

    /// Settings
    #[serde(default)]
    pub settings: RoleSettings,

    /// System packages
    #[serde(default)]
    pub system: HashMap<String, SystemPackageValue>,

    /// Tools
    #[serde(default)]
    pub tools: HashMap<String, String>,
}

impl RoleConfig {
    pub fn load(config_file: PathBuf) -> Result<Self> {
        let contents = fs::read_to_string(&config_file).wrap_err(format!(
            "failed to read package config: {}",
            config_file.display()
        ))?;
        let config: Self = toml::from_str(contents.as_str())
            .wrap_err(format!("failed to parse TOML: {}", config_file.display()))?;

        Ok(config)
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct RoleSettings {
    /// Remove empty directories when uninstalling.
    #[serde(default = "default_to_true")]
    pub remove_empty_dir: bool,
}

fn default_to_true() -> bool {
    true
}

impl Default for RoleSettings {
    fn default() -> Self {
        Self {
            remove_empty_dir: true,
        }
    }
}
