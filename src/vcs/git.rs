use std::path::PathBuf;

use color_eyre::eyre::Result;

use super::Vcs;

struct Git {}

impl Vcs for Git {
    fn get(&self, url: String, name: String) -> Result<PathBuf> {
        let path = PathBuf::new();

        Ok(path)
    }
}
