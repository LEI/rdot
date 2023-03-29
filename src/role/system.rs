use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct SystemPackage {
    /// System package version.
    #[serde(default = "default_version")]
    pub(crate) version: String,

    /// Prevents the package from being removed.
    #[serde(default)]
    pub(crate) keep: bool,
}

fn default_version() -> String {
    "latest".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum SystemPackageValue {
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
