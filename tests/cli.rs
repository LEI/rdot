use assert_cmd::Command;
use predicates::prelude::*;

fn bin() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

#[test]
fn help_command() {
    bin()
        .arg("--dry-run")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn list_command() {
    bin()
        .arg("list")
        .arg("--dry-run")
        .arg("--config-file=examples/config.toml")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available"));
}

#[test]
fn install_command() {
    bin()
        .arg("install")
        .arg("--dry-run")
        .arg("--config-file=examples/config.toml")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dry-run"));
}

#[test]
fn remove_command() {
    bin()
        .arg("remove")
        .arg("--dry-run")
        .arg("--config-file=examples/config.toml")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dry-run"));
}
