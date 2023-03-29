use std::collections::HashMap;

use serde::Deserialize;

use super::system::SystemPackageValue;

#[derive(Debug, Default, Deserialize)]
pub(crate) struct RoleConfig {
    /// Dependencies
    #[serde(default)]
    pub(crate) dependencies: HashMap<String, String>,

    /// Environment
    #[serde(default)]
    pub(crate) env: HashMap<String, String>,

    /// Lines
    #[serde(default)]
    pub(crate) lines: HashMap<String, String>,

    /// Links
    #[serde(default)]
    pub(crate) links: HashMap<String, String>,

    /// Settings
    #[serde(default)]
    pub(crate) settings: RoleSettings,

    /// System packages
    #[serde(default)]
    pub(crate) system: HashMap<String, SystemPackageValue>,

    /// Tools
    #[serde(default)]
    pub(crate) tools: HashMap<String, String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub(crate) struct RoleSettings {
    /// Remove empty directories when uninstalling.
    #[serde(default = "default_to_true")]
    pub(crate) remove_empty_dir: bool,
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
