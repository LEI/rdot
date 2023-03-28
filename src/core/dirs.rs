// https://github.com/jdxcode/rtx/blob/main/src/dirs.rs
use std::path::PathBuf;

use lazy_static::lazy_static;

use crate::core::env;

lazy_static! {
    pub static ref CURRENT: PathBuf = env::PWD.clone();
    pub static ref HOME: PathBuf = env::HOME.clone();
    pub static ref ROOT: PathBuf = env::RDOT_DATA_DIR.clone();
    pub static ref CACHE: PathBuf = env::RDOT_CACHE_DIR.clone();
    pub static ref CONFIG: PathBuf = env::RDOT_CONFIG_DIR.clone();
    pub static ref ROLES: PathBuf = env::RDOT_DATA_DIR.join("roles");
    // pub static ref DOWNLOADS: PathBuf = env::RDOT_DATA_DIR.join("downloads");
    // pub static ref INSTALLS: PathBuf = env::RDOT_DATA_DIR.join("installs");
}
