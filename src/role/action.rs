use std::fmt::Display;
use std::path::PathBuf;

use color_eyre::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    config::{os::OsValue, RoleValue},
    role::{config::RoleConfig, line::Line, link::Link, system::SystemPackage},
    rtx::Rtx,
    system::System,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Action {
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

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Role {
    /// The name of the package to install
    // This is hidden because the map key contains the name
    #[serde(skip)]
    pub(crate) name: String,

    /// Supported operating systems
    #[serde(default)]
    pub(crate) os: Option<OsValue>,

    /// The path to the role directory
    pub(crate) path: PathBuf,
}

impl Role {
    /// Installs a role with a given configuration.
    pub(crate) fn install_or_remove(
        &self,
        action: Action,
        config: RoleConfig,
        dry_run: bool,
        verbose: u8,
    ) -> Result<()> {
        println!("--- {:?} role: {}", action, self.name);

        let role_settings = config.settings;
        if verbose > 0 {
            println!("Role settings: {:#?}", role_settings);
        }

        for os in &config.os {
            println!("# OS: {}", os);
        }

        for dep in &config.dependencies {
            println!("# TODO: import {} from {}", dep.0, dep.1);
        }

        // println!("Env: {:?}", config.env);
        for var in &config.env {
            println!("# TODO: export {}={}", var.0, var.1);
        }

        // println!("Lines: {:?}", config.lines);
        for value in &config.lines {
            let line: Line = value.into();

            line.execute(action, self.path.to_path_buf(), dry_run)?;
        }

        // println!("Links: {:?}", config.links);
        for value in &config.links {
            let link: Link = value.into();

            link.execute(action, self.path.to_path_buf(), dry_run, role_settings)?;
        }

        for package in &config.system {
            let sp = SystemPackage::from(package.1);
            let package = format!("{}@{:?}", package.0, sp.version);
            let system = System::default();

            println!("system {} {}", action, package);
            match action {
                Action::Install => {
                    if !dry_run {
                        system.install(&package);
                    }
                }
                Action::Remove => {
                    if !dry_run && !sp.keep {
                        system.remove(&package);
                    }
                }
            }
        }

        // println!("Tools: {:?}", config.tools);
        for tool in &config.tools {
            let tool = format!("{}@{}", tool.0, tool.1);
            let rtx = Rtx::default();

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

        println!("--- Done");
        Ok(())
    }

    // fn status(&self) -> RoleStatus {
    //     let installed = false;

    //     RoleStatus {
    //         installed,
    //         path: self.path.to_path_buf(),
    //     }
    // }
}

// #[derive(Debug)]
// struct RoleStatus {
//     installed: bool,
//     path: PathBuf,
// }

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} {:?}",
            self.name,
            self.path.display(),
            self.os // .join(",")
        )
    }
}

impl From<(&String, &RoleValue)> for Role {
    fn from(value: (&String, &RoleValue)) -> Self {
        let name = value.0.to_string();

        let mut role: Role = value.1.into();
        role.name = name;

        role
    }
}

impl From<&RoleValue> for Role {
    fn from(value: &RoleValue) -> Self {
        match value {
            RoleValue::String(value) => Role {
                // name: value.to_string(),
                path: PathBuf::from(value),
                ..Default::default()
            },
            RoleValue::Table(value) => value.clone(),
        }
    }
}
