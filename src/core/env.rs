// https://github.com/jdxcode/rtx/blob/main/src/env.rs
// pub use std::env::*;
use std::env::*;
use std::path::PathBuf;

lazy_static! {
    pub static ref HOME: PathBuf = dirs_next::home_dir().unwrap();
    pub static ref PWD: PathBuf = current_dir().unwrap();
    pub static ref XDG_CACHE_HOME: PathBuf =
        dirs_next::cache_dir().unwrap_or_else(|| HOME.join(".cache"));
    pub static ref XDG_CONFIG_HOME: PathBuf =
        var_path("XDG_CONFIG_HOME").unwrap_or_else(|| HOME.join(".config"));
    pub static ref XDG_DATA_HOME: PathBuf =
        var_path("XDG_DATA_HOME").unwrap_or_else(|| HOME.join(".local/share"));
    pub static ref RDOT_CACHE_DIR: PathBuf =
        var_path("RDOT_CACHE_DIR").unwrap_or_else(|| XDG_CACHE_HOME.join("rdot"));
    pub static ref RDOT_CONFIG_DIR: PathBuf =
        var_path("RDOT_CONFIG_DIR").unwrap_or_else(|| XDG_CONFIG_HOME.join("rdot"));
    pub static ref RDOT_DATA_DIR: PathBuf =
        var_path("RDOT_DATA_DIR").unwrap_or_else(|| XDG_DATA_HOME.join("rdot"));
    pub static ref RDOT_DEBUG: bool = var_is_true("RDOT_DEBUG");
    pub static ref SHELL: String = var("SHELL").unwrap_or_else(|_| "sh".into());
    pub static ref CI: bool = var_is_true("CI");
}

fn var_is_true(key: &str) -> bool {
    match var(key) {
        Ok(v) => {
            let v = v.to_lowercase();
            !v.is_empty()
                && v != "n"
                && v != "no"
                && v != "false"
                && v != "0"
                && v != "off"
                && v != " "
        }
        Err(_) => false,
    }
}

fn var_path(key: &str) -> Option<PathBuf> {
    var_os(key).map(PathBuf::from)
}
