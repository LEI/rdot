use color_eyre::{eyre::eyre, Result};

use crate::core::{config::Config, role::Role};

#[macro_use]
extern crate lazy_static;

pub mod cli;
pub mod core;
pub mod package;
pub mod rtx;
pub mod system;

// TODO: move to filter_roles
pub fn filter(config: &Config, filter: &Vec<String>) -> Result<Vec<Role>> {
    if filter.is_empty() {
        return Ok(config.roles.iter().map(|role| role.into()).collect());
    }

    let mut result: Vec<Role> = vec![];

    for arg in filter {
        let role = config.roles.iter().find(|role| role.0 == arg);
        if role.is_none() {
            return Err(eyre!("Invalid role name in arguments: {}", arg));
        }
        result.push(role.unwrap().into());
    }

    Ok(result)
}
