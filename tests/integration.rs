use assert_cmd::Command;
use predicates::prelude::*;

const DRY_RUN: &str = "Dry-run";
const USAGE: &str = "Usage:";
const ROLE_COUNT: usize = 4;

fn bin() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

#[test]
fn root_command() {
    bin()
        .arg("--dry-run")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(USAGE));
}

#[test]
fn help_command() {
    bin()
        .arg("help")
        .assert()
        .success()
        .stdout(predicate::str::contains(USAGE));
}

#[test]
fn init_command() {
    bin()
        .arg("init")
        .arg("--config=none")
        .arg("--dry-run")
        .arg("--yes")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));
}

#[test]
fn list_command() {
    bin()
        .arg("list")
        .arg("git")
        .arg("--config=examples")
        .arg("--dry-run")
        .arg("--format=raw")
        .assert()
        .success()
        .stdout(predicate::str::is_match("^Dry-run\ngit \\./git\n$").unwrap());
}

#[test]
fn list_command_env() {
    bin()
        .arg("list")
        .arg("--dry-run")
        .env("RDOT_CONFIG_DIR", "examples")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Available roles: {}\n",
            ROLE_COUNT
        )));
}

#[test]
fn install_command() {
    bin()
        .arg("--dry-run")
        .arg("install")
        .arg("--config=examples")
        .assert()
        .success()
        .stdout(predicate::str::contains(DRY_RUN));
}

#[test]
fn remove_command() {
    bin()
        .arg("--dry-run")
        .arg("remove")
        .arg("--config=examples")
        .assert()
        .success()
        .stdout(predicate::str::contains(DRY_RUN));
}

#[test]
fn sync_command() {
    bin()
        .arg("sync")
        .arg("--config=examples")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::is_match("Dry-run").unwrap());
}
