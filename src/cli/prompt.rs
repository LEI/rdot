use color_eyre::eyre::{Context, Result};
use dialoguer::Confirm;

pub(crate) fn confirm(prompt: String) -> Result<bool> {
    Confirm::new()
        .with_prompt(prompt)
        .interact()
        .wrap_err("Failed to confirm prompt")
}
