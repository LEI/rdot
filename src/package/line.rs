use std::{fmt::Display, fs::read_to_string, path::PathBuf};

use color_eyre::{eyre::eyre, Result};

use crate::{core::dirs, package::Action};

pub struct Line {
    /// The path to the target file
    pub file: PathBuf,

    // The line to append or remove
    pub line: String,
    // after, always_after?
}

impl Line {
    pub fn execute(&self, action: Action, _directory: PathBuf, dry_run: bool) -> Result<()> {
        let line = &self.line;
        let mut target = &self.file;
        let path: PathBuf;
        if target.starts_with("~") {
            let home_dir = dirs::HOME.to_path_buf();
            path = home_dir.join(target.strip_prefix("~").unwrap());

            target = &path;
        };

        // Verify
        // let parent_dir = target.parent().unwrap();
        // let create_parent_dir = !parent_dir.is_dir();

        if line.is_empty() {
            return Err(eyre!("Empty line"));
        }

        if target.exists() {
            // let metadata = fs::symlink_metadata(target);
            // println!("Metadata: {:#?}", metadata);

            match action {
                Action::Install => {
                    let contents = read_to_string(target)?;
                    // TODO: diff println!("{} = {}", contents, line);
                    if contents.contains(line) {
                        println!("Line already in file: {}", line);
                        return Ok(());
                    }
                }
                Action::Remove => {}
            }
        }

        // Print
        match action {
            Action::Install => {
                println!("TODO: echo {:?} >> {:?}", line, target);
            }
            Action::Remove => {
                println!("TODO rm line sed /{:?}/d {:?}", line, target);
            }
        }

        if dry_run {
            return Ok(());
        }

        // Execute
        match action {
            Action::Install => {
                // if create_parent_dir {
                //     create_dir(parent_dir).wrap_err("failed to create dir")?;
                // }

                // // TODO: windows
                // os::unix::fs::symlink(source, target).wrap_err("failed to symlink")
                Ok(())
            }
            Action::Remove => {
                // let result = if target.exists() {
                //     remove_file(target).wrap_err("failed to unlink")
                // } else {
                //     Ok(())
                // };

                // if settings.remove_empty_dir
                //     && parent_dir.is_dir()
                //     && parent_dir.read_dir()?.next().is_none()
                // {
                //     remove_dir(parent_dir).wrap_err("failed to remove dir")?;
                // }

                // result
                Ok(())
            }
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} >> {:?}", self.line, self.file)
    }
}

impl From<(&String, &String)> for Line {
    fn from(value: (&String, &String)) -> Self {
        Self {
            file: PathBuf::from(value.0),
            line: value.1.to_string(),
        }
    }
}
