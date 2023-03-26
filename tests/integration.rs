// use std::path::PathBuf;

// use core;

use std::{collections::HashMap, path::PathBuf};

use rdot::{cli::PackageArgs, core::config::Config, filter, package::Package};

#[test]
fn integration() {
    env_logger::init();
    let path = PathBuf::from("examples/config.toml");
    let config = Config::load(&path).expect("failed to load config");
    log::debug!("Loaded global config: {:#?}", config);
    assert_eq!(config.env, HashMap::new());
    assert_eq!(
        config.packages,
        HashMap::from([
            ("git".to_string(), "./git".to_string()),
            ("rtx".to_string(), "./rtx".to_string()),
            ("starship".to_string(), "./starship".to_string()),
        ])
    );

    let packages = filter(
        &config,
        &PackageArgs {
            filter: vec![],
            sync: false,
        },
    )
    .unwrap();
    Package::list(&packages, false).unwrap();
    // TODO: capture stdout
}
