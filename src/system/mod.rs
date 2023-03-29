use std::process::{Command, Output};

#[derive(Default)]
pub(crate) struct System {}

impl System {
    pub(crate) fn install(&self, pkg: &str) -> Output {
        self.manager()
            .arg("install")
            .arg(pkg)
            .output()
            .expect("failed to execute system install")
    }

    pub(crate) fn remove(&self, pkg: &str) -> Output {
        self.manager()
            .arg("remove")
            .arg(pkg)
            .output()
            .expect("failed to execute system remove")
    }

    fn manager(&self) -> Command {
        let manager = PackageManager::new();

        std::process::Command::new(manager.name)
    }
}

struct PackageManager {
    name: String,
    // TODO: commands: HashMap<Action, String>,
}

impl PackageManager {
    pub(crate) fn new() -> Self {
        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        let name = "apt-get"; // FIXME: detect distrib
        #[cfg(target_os = "macos")]
        let name = "brew";
        #[cfg(target_os = "windows")]
        let name = "scoop";

        Self {
            name: name.to_string(),
            // commands,
        }
    }
}
