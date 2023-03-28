use std::{collections::HashMap, path::PathBuf};

// use rdot::bin;
use rdot::{
    core::{config::Config, role::Role},
    filter,
};

#[test]
fn integration() {
    color_eyre::install().unwrap();
    env_logger::init();

    // bin::rdot().unwrap()

    let path = PathBuf::from("examples/config.toml");
    let config = Config::load(&path).expect("failed to load config");
    log::debug!("Loaded global config: {:#?}", config);
    assert_eq!(config.env, HashMap::new());
    assert!(config.roles.iter().len() > 0);

    let roles = filter(&config, &vec![]).unwrap();
    Role::list(&roles, false).unwrap();
    // TODO: capture stdout
}
