#[cfg(test)]
mod mysql_failure_path_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    /// should be using all CLI args to provide credential for DB connection
    #[test]
    fn failure_with_all_cli_args() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

        cmd.arg("samples/generic/js-failure-path1")
            .arg("--ext=js")
            .arg("--db-type=mysql")
            .arg("--db-host=127.0.0.1")
            .arg("--db-port=33306")
            .arg("--db-user=root")
            .arg("--db-name=sqlx-ts");
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains(
                "Table \'sqlx-ts.unknown\' doesn\'t exist",
            ))
            .stdout(predicates::str::contains("SQLs failed to compile!"));

        Ok(())
    }
}
