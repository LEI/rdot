use color_eyre::{eyre::eyre, Result};

use crate::cli::PackageArgs;
use crate::core::config::Config;
use crate::package::Package;

#[macro_use]
extern crate lazy_static;

pub mod cli;
pub mod core;
pub mod package;
pub mod rtx;
pub mod system;

/// Filters the configured packages according to the provided filter in arguments.
///
/// # Examples
///
/// Valid filter (none means all).
///
/// ```
/// use std::collections::HashMap;
///
/// use rdot::cli::PackageArgs;
/// use rdot::core::config::Config;
///
/// let config = Config {
///   env: HashMap::new(),
///   packages: HashMap::from([
///     ("test".to_string(), "./examples/test".to_string())
///   ]),
///   path: std::path::PathBuf::new(),
/// };
///
/// let result = rdot::filter(&config, &PackageArgs{
///   filter: vec![],
///   sync: false,
/// });
/// assert_eq!(result.unwrap().len(), 1);
///
/// let result = rdot::filter(&config, &PackageArgs{
///   filter: vec!["test".to_string()],
///   sync: false,
/// });
/// assert_eq!(result.unwrap().len(), 1);
/// ```
///
/// # Panics
///
/// Invalid filter.
///
/// ```rust,should_panic
/// use rdot::cli::PackageArgs;
/// use rdot::core::config::Config;
///
/// let result = rdot::filter(&Config::default(), &PackageArgs{
///   filter: vec!["invalid".to_string()],
///   sync: false,
/// });
/// assert_eq!(result.unwrap().len(), 1);
/// ```
pub fn filter(config: &Config, args: &PackageArgs) -> Result<Vec<Package>> {
    if args.filter.is_empty() {
        return Ok(config
            .packages
            .iter()
            .map(|package| package.into())
            .collect());
    }

    let mut result: Vec<Package> = vec![];

    for arg in &args.filter {
        let package = config.packages.iter().find(|package| package.0 == arg);
        if package.is_none() {
            return Err(eyre!("Invalid package in arguments: {}", arg));
        }
        result.push(package.unwrap().into());
    }

    Ok(result)
}
