use std::{collections::HashMap, fs, path::PathBuf};

use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct PackageConfig {
    /// Dependencies
    pub dependencies: Option<HashMap<String, String>>,

    /// Environment
    pub env: Option<HashMap<String, String>>,

    /// Lines
    pub lines: Option<HashMap<String, String>>,

    /// Links
    pub links: Option<HashMap<String, String>>,

    /// Settings
    pub settings: Option<PackageSettings>,

    /// Tools
    pub tools: Option<HashMap<String, String>>,
}

impl PackageConfig {
    pub fn load(config_file: PathBuf) -> Result<Self> {
        let contents = fs::read_to_string(config_file)?;
        let config: Self = toml::from_str(contents.as_str())?;

        Ok(config)
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct PackageSettings {
    /// Remove empty directories when uninstalling.
    #[serde(default = "default_to_true")]
    pub remove_empty_dir: bool,

    /// Package action verbosity.
    #[serde(default)]
    pub verbose: bool,
}

fn default_to_true() -> bool {
    true
}

impl Default for PackageSettings {
    fn default() -> Self {
        Self {
            remove_empty_dir: true,
            verbose: false,
        }
    }
}
