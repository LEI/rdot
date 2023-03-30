use assert_cmd::Command;
use predicates::prelude::*;

const ROLE_COUNT: usize = 4;

fn bin() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

#[test]
fn no_arguments() {
    bin()
        .arg("--dry-run")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn help_command() {
    bin()
        .arg("--dry-run")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn init_command() {
    let new_config_file = "example/init.toml";
    bin()
        .arg("--dry-run")
        .arg("init")
        .arg("--config-file")
        .arg(new_config_file)
        .arg("--yes")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));
}

#[test]
fn list_command() {
    bin()
        .arg("--dry-run")
        .arg("list")
        .arg("git")
        .arg("--config-file=examples/config.toml")
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(format!(
                "^Dry-run(\n.*)*?
Available roles: {}
git: .*
$",
                ROLE_COUNT
            ))
            .unwrap(),
        );
}

#[test]
fn list_command_with_os() {
    bin()
        .arg("--dry-run")
        .arg("list")
        .arg("guix")
        .arg("--config-file=examples/config.toml")
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(format!(
                "^Dry-run(\n.*)*?
Available roles: {}
guix: .*Linux.*
$",
                ROLE_COUNT
            ))
            .unwrap(),
        );
}

#[test]
fn list_command_with_config_dir() {
    bin()
        .arg("--dry-run")
        .arg("list")
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
        .arg("--config-file=examples/config.toml")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dry-run"));
}

#[test]
fn remove_command() {
    bin()
        .arg("--dry-run")
        .arg("remove")
        .arg("--config-file=examples/config.toml")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dry-run"));
}
