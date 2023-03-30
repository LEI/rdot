use std::{env::set_var, fs, path::Path};

use color_eyre::eyre::{eyre, Context, Result};
use dialoguer::Confirm;

use crate::config::{expand, get_parent_dir, Config, SCHEMA_NAME};

use super::dirs;

/// Setup rdot env.
pub(crate) fn init_env() {
    let config_dir = dirs::CONFIG.to_path_buf();
    set_var("RDOT_CONFIG_DIR", config_dir);
}

/// Init dir options.
pub(crate) struct InitDir {
    pub(crate) confirm: bool,
    pub(crate) dry_run: bool,
    pub(crate) force: bool,
}

/// Setup config directory.
pub(crate) fn init_dir(config_file: &Path, options: InitDir) -> Result<Option<()>> {
    init_env();
    let config_file = expand(config_file)?;
    let parent_dir = get_parent_dir(&config_file)?;
    let schema_file = parent_dir.join(SCHEMA_NAME);

    if !options.force && !parent_dir.exists() {
        if options.confirm
            && !Confirm::new()
                .with_prompt(format!("Create directory: {}", parent_dir.display()))
                .interact()?
        {
            return Ok(None);
        }

        log::debug!("Creating directory: {}", parent_dir.display());
        if !options.dry_run {
            fs::create_dir(&parent_dir)?;
        }
    } else if !options.force && schema_file.exists() {
        return Err(eyre!(
            "Schema file already exists (use --force): {}",
            schema_file.display()
        ));
    }

    if options.confirm
        && !Confirm::new()
            .with_prompt(format!("Create schema: {}", schema_file.display()))
            .interact()?
    {
        return Ok(None);
    }

    log::debug!("Writing schema: {}", schema_file.display());
    let schema_json = Config::get_schema_json()?;
    if !options.dry_run {
        fs::write(&schema_file, schema_json)
            .wrap_err(format!("Failed to write schema: {}", schema_file.display()))?;
    }

    log::debug!("Writing default config: {}", config_file.display());
    let header = format!(
        "#:schema ./{}\n\n# os = []\n",
        schema_file.file_name().unwrap().to_string_lossy()
    );
    let toml = Config::get_default_toml()?;
    let config_json = format!("{}\n{}", header, toml);
    if !options.dry_run {
        fs::write(&config_file, config_json)
            .wrap_err(format!("Failed to write config: {}", config_file.display()))?;
    }

    Ok(Some(()))
}
