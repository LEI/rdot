use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Context, Result};
use serde::Deserialize;

use super::dirs;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// Environment
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Packages
    pub packages: HashMap<String, String>,

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

        let contents =
            fs::read_to_string(&config_file).wrap_err(format!("{}", &config_file.display()))?;
        let mut config: Config = toml::from_str(contents.as_str())?;
        config.path = config_file;

        Ok(config)
    }
}

// fn load_env() -> HashMap<String, String> {
//     let mut env = HashMap::new();

//     env
// }
