use std::process::{Command, Output};

#[derive(Default)]
pub(crate) struct Rtx {}

impl Rtx {
    // TODO: stdout and stderr handling
    // https://docs.rs/color-eyre/latest/color_eyre/#custom-sections-for-error-reports-via-section-trait

    pub(crate) fn install(&self, tool: &str) -> Output {
        self.rtx()
            .arg("global")
            .arg(tool)
            .output()
            .expect("failed to execute rtx global");

        self.rtx()
            .arg("install")
            .arg(tool)
            .output()
            .expect("failed to execute rtx install")
    }

    pub(crate) fn remove(&self, tool: &str) -> Output {
        self.rtx()
            .arg("global")
            .arg("--remove")
            .arg(tool)
            .output()
            .expect("failed to execute rtx global --remove");

        self.rtx()
            .arg("remove")
            .arg(tool)
            .output()
            .expect("failed to execute rtx install")
    }

    fn rtx(&self) -> Command {
        std::process::Command::new("rtx")
    }
}
