use std::{fs, path::PathBuf};

use color_eyre::eyre::{Context, Result};
use serde::de::DeserializeOwned;

pub(crate) fn load_toml<T>(file: &PathBuf) -> Result<T>
where
    T: DeserializeOwned,
{
    let contents = fs::read_to_string(file)?;

    toml::from_str(contents.as_str()).wrap_err("Invalid TOML")
}
