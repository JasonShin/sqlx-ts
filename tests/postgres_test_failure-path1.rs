use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

/// should be using all CLI args to provide credential for DB connection
#[test]
fn failure_with_all_cli_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

    cmd.arg("samples/generic/failure-path1")
        .arg("--db-host=localhost")
        .arg("--db-port=54321")
        .arg("--db-user=postgres")
        .arg("--db-pass=postgres");
    cmd.assert()
        .failure()
        .stdout(predicates::str::contains("SQLs failed to compile!"));

    Ok(())
}
