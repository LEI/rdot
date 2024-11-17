use assert_cmd::Command;
use color_eyre::eyre::Result;
use predicates::prelude::*;

const DRY_RUN: &str = "Dry-run";
const USAGE: &str = "Usage:";
const ROLE_COUNT: usize = 4;

fn bin() -> Result<Command> {
    let name = "main"; // env!("CARGO_PKG_NAME");
    let cmd = Command::cargo_bin(name)?;

    Ok(cmd)
}

#[test]
fn root_command() -> Result<()> {
    bin()?
        .arg("--dry-run")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(USAGE));

    Ok(())
}

#[test]
fn help_command() -> Result<()> {
    bin()?
        .arg("help")
        .assert()
        .success()
        .stdout(predicate::str::contains(USAGE));

    Ok(())
}

#[test]
fn init_command() -> Result<()> {
    bin()?
        .arg("init")
        .arg("--config=none")
        .arg("--dry-run")
        .arg("--yes")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    Ok(())
}

#[test]
fn list_command() -> Result<()> {
    bin()?
        .arg("list")
        .arg("git")
        .arg("--config=examples")
        .arg("--dry-run")
        .arg("--format=raw")
        .assert()
        .success()
        .stdout(predicate::str::is_match(format!(
            // "^{}\ngit \\./git\n$",
            // FIXME:
            "^{}\n\\./git \\./git\n$",
            DRY_RUN
        ))?);

    Ok(())
}

#[test]
fn list_command_env() -> Result<()> {
    bin()?
        .arg("list")
        .arg("--dry-run")
        .env("RDOT_CONFIG_DIR", "examples")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Available roles: {}\n",
            ROLE_COUNT
        )));

    Ok(())
}

#[test]
fn install_command() -> Result<()> {
    bin()?
        .arg("--dry-run")
        .arg("install")
        .arg("--config=examples")
        .assert()
        .success()
        .stdout(predicate::str::contains(DRY_RUN));

    Ok(())
}

#[test]
fn remove_command() -> Result<()> {
    bin()?
        .arg("--dry-run")
        .arg("remove")
        .arg("--config=examples")
        .assert()
        .success()
        .stdout(predicate::str::contains(DRY_RUN));

    Ok(())
}

#[test]
fn sync_command() -> Result<()> {
    bin()?
        .arg("sync")
        .arg("--config=examples")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::is_match(DRY_RUN)?);

    Ok(())
}
