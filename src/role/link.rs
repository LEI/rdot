use std::{
    fmt::Display,
    fs::{create_dir, read_link, remove_dir, remove_file},
    os,
    path::PathBuf,
};

use color_eyre::{
    eyre::{eyre, Context},
    Result,
};

use crate::{core::dirs, role::Action};

use super::config::RoleSettings;

pub struct Link {
    /// The path to the source file
    pub original: PathBuf,

    /// The path to the target file
    pub link: PathBuf,
}

impl Link {
    pub fn execute(
        &self,
        action: Action,
        directory: PathBuf,
        dry_run: bool,
        settings: RoleSettings,
    ) -> Result<()> {
        let source = if self.original.is_relative() {
            directory.join(&self.original)
        } else {
            self.original.to_path_buf()
        };
        let mut target = &self.link;
        let path: PathBuf;
        if target.starts_with("~") {
            let home_dir = dirs::HOME.to_path_buf();
            path = home_dir.join(target.strip_prefix("~").unwrap());
            target = &path;
        };
        let parent_dir = target.parent().unwrap();
        let create_parent_dir = !parent_dir.is_dir();

        // Print
        match action {
            Action::Install => {
                if create_parent_dir {
                    println!("mkdir {:?}", parent_dir);
                }
                println!("ln -s {:?} {:?}", source, target);
            }
            Action::Remove => {
                println!("rm {:?}", target);
                if settings.remove_empty_dir {
                    println!("rmdir {:?}", parent_dir);
                }
            }
        }

        // Verify
        if !source.exists() {
            return Err(eyre!("Source file not found: {}", source.display()));
        }

        if target.exists() {
            // let metadata = fs::symlink_metadata(target);
            // println!("Metadata: {:#?}", metadata);

            match action {
                Action::Install => {
                    if let Ok(path) = read_link(target) {
                        if path != source {
                            return Err(eyre!(
                                "Invalid target, another link exists: {} -> {}",
                                target.display(),
                                path.display()
                            ));
                        }
                        println!("# Target already installed: {}", target.display());

                        return Ok(());
                    }
                }
                Action::Remove => {
                    if let Ok(path) = read_link(target) {
                        if path != source {
                            return Err(eyre!(
                                "Invalid target, another link exists: {} -> {}",
                                target.display(),
                                path.display()
                            ));
                        }
                    }
                    if !target.exists() {
                        println!("# Target already removed: {}", target.display());

                        return Ok(());
                    }
                }
            }
        }

        if dry_run {
            return Ok(());
        }

        // Execute
        match action {
            Action::Install => {
                if create_parent_dir {
                    create_dir(parent_dir).wrap_err("failed to create dir")?;
                }

                #[cfg(target_os = "windows")]
                os::windows::fs::symlink_file(source, target).wrap_err("failed to symlink")?;
                #[cfg(not(target_os = "windows"))]
                os::unix::fs::symlink(source, target).wrap_err("failed to symlink")?;

                Ok(())
            }
            Action::Remove => {
                let result = if target.exists() {
                    remove_file(target).wrap_err("failed to unlink")
                } else {
                    Ok(())
                };

                if settings.remove_empty_dir
                    && parent_dir.is_dir()
                    && parent_dir.read_dir()?.next().is_none()
                {
                    remove_dir(parent_dir).wrap_err("failed to remove dir")?;
                }

                result
            }
        }
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.original.display(), self.link.display())
    }
}

impl From<(&String, &String)> for Link {
    fn from(value: (&String, &String)) -> Self {
        Self {
            original: PathBuf::from(value.0),
            link: PathBuf::from(value.1),
        }
    }
}
