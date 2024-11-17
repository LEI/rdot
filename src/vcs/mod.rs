use std::path::PathBuf;

use color_eyre::eyre::Result;

mod git;

/// Describes a CLI command.
pub(crate) trait Vcs: Sized {
    /// Clone a remote repository.
    fn get(&self, url: String, name: String) -> Result<PathBuf>;
}

pub(crate) fn parse(spec: String) -> (String, String) {
    let url = spec;
    let name = "".to_string();

    (url, name)
}
