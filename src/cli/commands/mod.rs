use std::io::StdoutLock;

use color_eyre::eyre::Result;

use crate::config::Config;

use super::GlobalOptions;

pub(crate) mod init;
pub(crate) mod install;
pub(crate) mod list;
pub(crate) mod remove;

/// Describes a CLI command.
pub trait Command: Sized {
    /// CLI command entry point.
    fn run(self, config: &Config, options: GlobalOptions, stdout: &mut StdoutLock) -> Result<()>;
}
