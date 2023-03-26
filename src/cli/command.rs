use std::io::StdoutLock;

use color_eyre::eyre::Result;

use crate::core::config::Config;
// use crate::output::Output;

use super::PackageArgs;

/// Describes a CLI command.
pub trait Command: Sized {
    /// CLI command entry point.
    fn run(self, args: &PackageArgs, config: Config, stdout: &mut StdoutLock) -> Result<()>;
}
