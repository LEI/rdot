use std::io::StdoutLock;

use color_eyre::eyre::Result;

use crate::config::Config;

use super::Options;

pub(crate) mod init;
pub(crate) mod install;
pub(crate) mod list;
pub(crate) mod remove;
pub(crate) mod sync;

/// Describes a CLI command.
pub trait Command: Sized {
    /// CLI command entry point.
    fn run(self, config: &Config, options: Options, stdout: &mut StdoutLock) -> Result<()>;
}
