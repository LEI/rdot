use std::{
    env::{set_var, var},
    fs,
    path::Path,
};

#[cfg(feature = "json-schema")]
use color_eyre::eyre::eyre;
use color_eyre::eyre::{Context, Result};

#[cfg(feature = "json-schema")]
use crate::config::SCHEMA_NAME;
use crate::{
    cli::prompt,
    config::{expand, Config},
};

use super::dirs;

/// Setup the global environment variables.
pub(crate) fn init_env() {
    if var("RDOT_CONFIG_DIR").is_err() {
        let config_dir = dirs::CONFIG.to_path_buf();
        set_var("RDOT_CONFIG_DIR", config_dir);
    }
}

/// Init dir options.
// TODO: refactor confirm/force/yes options
pub(crate) struct InitDirOptions {
    /// Wether to prompt for confirmation.
    pub(crate) confirm: bool,
    /// Only print.
    pub(crate) dry_run: bool,
    /// Overrides schema.
    #[cfg(feature = "json-schema")]
    pub(crate) force: bool,
}

/// Setup config directory.
pub(crate) fn init_dir(config_dir: &Path, options: InitDirOptions) -> Result<Option<()>> {
    let config_file = config_dir.join("config.toml");
    let config_file = expand(&config_file)?;
    #[cfg(feature = "json-schema")]
    let schema_file = config_dir.join(SCHEMA_NAME);

    let _new_dir = if !config_dir.exists() {
        if options.confirm
            && !prompt::confirm(format!("Create directory: {}", config_dir.display()))?
        {
            return Ok(None);
        }

        debug!("Creating directory: {}", config_dir.display());
        if !options.dry_run {
            fs::create_dir(config_dir)?;
        }

        true
    } else {
        false
    };

    #[cfg(feature = "json-schema")]
    {
        if _new_dir && !options.force && schema_file.exists() {
            return Err(eyre!(
                "Schema file already exists (use --force): {}",
                schema_file.display()
            ));
        }

        if !options.confirm || prompt::confirm(format!("Create schema: {}", schema_file.display()))?
        {
            debug!("Writing schema: {}", schema_file.display());
            let schema_json = Config::get_schema_json()?;
            if !options.dry_run {
                fs::write(&schema_file, schema_json)
                    .wrap_err(format!("Failed to write schema: {}", schema_file.display()))?;
            }
        }
    }

    debug!("Writing default config to: {}", config_file.display());
    #[cfg(feature = "json-schema")]
    let header = format!(
        "#:schema ./{}\n\n# os = []\n",
        schema_file.file_name().unwrap().to_string_lossy()
    );
    let toml = Config::get_default_toml()?;
    #[cfg(feature = "json-schema")]
    let toml = format!("{}\n{}", header, toml);
    if !options.dry_run {
        fs::write(&config_file, toml)
            .wrap_err(format!("Failed to write config: {}", config_file.display()))?;
    }

    Ok(Some(()))
}
