use std::{collections::HashMap, fs, path::PathBuf};

use color_eyre::eyre::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct PackageConfig {
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
    pub settings: PackageSettings,

    /// System packages
    #[serde(default)]
    pub system: HashMap<String, SystemPackageValue>,

    /// Tools
    #[serde(default)]
    pub tools: HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SystemPackage {
    /// System package version.
    #[serde(default = "default_version")]
    pub version: String,

    /// Prevents the package from being removed.
    #[serde(default)]
    pub keep: bool,
}

fn default_version() -> String {
    "latest".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SystemPackageValue {
    /// Represents a TOML string
    String(String),
    // /// Represents a TOML integer
    // Integer(i64),
    // /// Represents a TOML float
    // Float(f64),
    // /// Represents a TOML boolean
    // Boolean(bool),
    // /// Represents a TOML datetime
    // Datetime(Datetime),
    // /// Represents a TOML array
    // Array(Array),
    /// Represents a TOML table
    Table(SystemPackage),
}

impl From<&SystemPackageValue> for SystemPackage {
    fn from(value: &SystemPackageValue) -> Self {
        match value {
            SystemPackageValue::String(value) => SystemPackage {
                version: value.to_string(),
                ..Default::default()
            },
            SystemPackageValue::Table(value) => value.clone(),
        }
    }
}

impl PackageConfig {
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
pub struct PackageSettings {
    /// Remove empty directories when uninstalling.
    #[serde(default = "default_to_true")]
    pub remove_empty_dir: bool,
}

fn default_to_true() -> bool {
    true
}

impl Default for PackageSettings {
    fn default() -> Self {
        Self {
            remove_empty_dir: true,
        }
    }
}
