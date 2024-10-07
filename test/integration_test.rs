// tests/integration\_test.rs
use std::process::Command; // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
#[test]
fn run_with_defaults() {
    Command::cargo_bin("horse_say")
        .expect("binary exists")
        .assert()
        .success();
}

#[test]
fn run_with_defaults() {
    Command::cargo_bin("horse_say")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));
        .failure();
    Ok(())
}
