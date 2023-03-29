// https://github.com/jdxcode/rtx/blob/main/src/dirs.rs
use std::path::PathBuf;

use lazy_static::lazy_static;

use super::env;

lazy_static! {
    pub(crate) static ref CURRENT: PathBuf = env::PWD.clone();
    pub(crate) static ref HOME: PathBuf = env::HOME.clone();
    pub(crate) static ref ROOT: PathBuf = env::RDOT_DATA_DIR.clone();
    pub(crate) static ref CACHE: PathBuf = env::RDOT_CACHE_DIR.clone();
    pub(crate) static ref CONFIG: PathBuf = env::RDOT_CONFIG_DIR.clone();
    pub(crate) static ref ROLES: PathBuf = env::RDOT_DATA_DIR.join("roles");
    // pub(crate) static ref DOWNLOADS: PathBuf = env::RDOT_DATA_DIR.join("downloads");
    // pub(crate) static ref INSTALLS: PathBuf = env::RDOT_DATA_DIR.join("installs");
}
